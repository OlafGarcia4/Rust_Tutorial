# Concurrency

One of Rust goals is to handle concurrent programming efficiently and safe. Concurrent and parallel programming are closely related since concurrent programming is a program where differnt parts of the code can run idenpendently and parallel programming is when differnt code parts run at the same time. Because Rust ownership and type checking, it allows concurrent programming to be done without issues since the program will not compile if it fails, this has been nicknamed **Fearless Concurrency**.

## Threads

Threads are responsible for running our programs. Using more threads will help performance at the cost of complexity. This comes with some issues like:

- Race Conditions -  Data/Resources being accessed inconsistently by multiple threads
- Deadlocks - When 2 threads are waiting for a resource that the other thread is using, leading to stalemate as neither can complete its process 
- Execution order - The process passed to threads  are non-deterministic, meaning bugs can happen when only when certain conditions are met.

----------------------------------------------------------------
One-to-one (OtO) vs Green thread

OtO threads that map to a thread directly onto a thread on the operating system while green threads are multiple threads that can map to 1 os thread. So we can have 20 green threads connect to 5 os threads. Rust is concerned about performace on run time, so it only provide support for OtO threads on the standard library, however we can get support for green threads with external creates. 

    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

The example above declares a thread along with for loop. Remember that Rust only supports OtO, so that means that in this code with the thread::sleep() function, working thread will switch between our declared thread and the thread for the 'for' loop. We can see this by running the code (Output might be different from that of the book, it all depends on how your OS handels threads). We can see that on the output we did not reached the end of the loop for our thread, this is beacuse the main thread with the for loop, ended and therefore end out thread. So how can we have our thread finish before the main thread ends?

We can use a JoinHandle<()> data type to store our thread

    fn main(){
        let handle = thread::spawn(||{...}) // or let handle: JoinHandle<()> 
        //...

        handle.join().unwrap();
    }

We can see that we stored the thread in handle, then we use the join() method to reuturn the Result, and then we use unwrap() to get the data with in the Result type. After we do thsi call our thread will run until completion. So if it is called after the for loop in main, the output will switch between our thread and the for loop until the loop ends and then the rest of our thread will finish. If we call it before the for loop, our thread will finish first and then the rest of main will finish.

This is the basic implementation of a thread. Now how can we pass data into a thread we created?

    use std::thread;

    fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn( || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

In this main function we try to use the vector and print out the data with in from out thread. However this is problematic. Since we can not specify the when the threads will execute, we run into validation issues. Our thread won't work since it cannot garantee that the vector will remain valid because the thread could run beyond the scope wher v is valid. This will happen to any data that is outside the closure of the thread. To avoid this we need to
specify that our thread takes ownership of v with the move keyword. This will stop us from using v outside the the thread scope.

    let handle = thread::spawn( move || { ... } );

## Message Passing

To ensure safe currency behavior, threads/actors will pass messages that contain data.

> *Do not communitcate by sharing memory, but share memory by communicating*

Rust achives this safety by creating Chanels for thread to communicate. A chanel consists of 2 parts, the transmitter and the reciever. This will allow for communication until either the transmitter or the reciever are dropped from memory.

    use std::sync::mpsc; // mpsc = multi producer, single consumer
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let recievedOne = rx.recv().unwrap();
        //let recievedTwo = rx.try_recv().unwrap();
    }

Here is a basic implemention of using message passing. We assign the tuple tx (Transmitter) and rx (Reciever) to our message passing instance. We can see we still need to use the move keyword to our thread closure because we want this thread to assume ownership to any related data in tx. We store a string in val, however we can store any type of data we want to pass, and then we pass the date downstream with the send() method. It important to note that we need to call the unwrap() method as send will return a Result type.

Now to get the message that is passed, we need to use the recv() or try_recv() method. The recv() will block the main thread until it has recieved a message, the try_recv() method will immediately return a Result type, where its a result or an error. The try_recv() method allows us to keep other thread running, and we can keep checking for a message if we use a loop.

As far ownership this is how it works. For example, if we try to use val after we send the message, we would get an error as send() will take ownership of the data. 

To pass multiple messages, we can treat rx as a vector for getting the methods and a iterator to pass the messages to tx:
    
    {
        thread::spawn(move || {
            let vals = vec![1,2,3,4];

            for val in vals{
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    for recieved in rx{
        println!("got: {}", recieved);
    }

To really use the multi producer part of it we can create another variable that clones the data from tx and repeat the same for loop as before. This way we can have tx and tx1 both send data to rx. 

------------------------------------------------------

## Sharing State

We have seen how we can run thread and pass information with chanels. essentialy "comunicating with out sharing data". However, this is not the only way to achive this. 

We can achive the oposite of this by using mutex. Mutual exclusion(mutex) allows for one thread to access any data at any time by using the a lock to secure the access. This locking system guards the data, this in turn become communicating by using memory.

There are 2 rules to using mutex.

1. A lock needs to be  acquired before the data can be used
1. The data must be unlocked once the working thread is finished.

------------------------------------------------------

    use std::sync::Mutex;
    fn main() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

 This is the basic declaration 
and implementartion for mutex. We can see that inorder to access the data in the mutex type varible we need to call for the lock() method, this will return a Result type, to get the data from this we call the unwrap() method.

Mutex is smart pointer(See the previous chapter 14), well, the lock() method retruns a smart pointer called MutexGuard wrapped in a LockResult. The lock we call for is dropped after the scope which it is in ends, taking care of the second rule for mutex. 

So how can we a Mutex variable with multiple threads?

    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

Well this an attempt, we can see that we we create 10 new threads and we store them in the vector handels. We can see that in handle's definition of the new threads not only take ownership of the data but also uses the lock system to manipulate the data. This function will not work, the error we get is for assigning the ownership of counter to multiple threads.

As we saw previously, a way to have multiple owners of data is by using Rc<T> (smart pointer) types. We can then wrap the mutex instace into Rc<T> and clone this smart pointer before we move  ownership. This would work but not for this case. In this instace we get another error that states that the passing Rc<Mutex\<i32>> can not be done safely. This is becaise the trait ***send*** is not part of a smart pointer. This happens becuase it is using the basic language primitives that can lead to bugs and memory leaks.

To solve this issue we have we need to use Atomic Reference Counting (Arc<T>). Arc<T> is a type similar to Rc<T> however it is created to be concurrent safe. Atomics are data primitives designed to be used in concurrent programming and can be called from the standar libry using **use std::sync::atomic**, see rust documentation for more details. So why not use Atomic types all the time? The reason comes down to performace, atomics enforce garantees that slow down the performace of threads. With this in mind, lets fix out previous issues.

    use std::sync::{Arc, Mutex};
    use std::thread;

    fn main() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    We can see that like with the Rc<T>, we wrap the Mutex around with Arc. This code will compile and return counter=10. The atomic module can provide us with more ways to access the primitive type, we only covered Mutex so we can see how it functions.

    