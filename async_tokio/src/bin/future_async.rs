
// make a multi threaded async executor with 10 worker threads
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // make 2 racers
    let racer01: F1Racer = F1Racer::new();
    let mut racer02: F1Racer = F1Racer::new();

    // name the other one something different and give him a faster lap time
    racer02.name = "Sergio Perez".to_string();
    racer02.lap_times.pop();
    racer02.lap_times.push(57);

    // put those tasks into two different threads
    let handler01 = tokio::task::spawn(racer01);
    let handler02 = tokio::task::spawn(racer02);

    loop {

        // once both tasks are finished print that
        if handler01.is_finished() && handler02.is_finished() {
            println!("All racers have finished!");        
            break;
        }

        // wait a little to not throttle the cpu
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

// Holds information on a racecar
struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}


// creates a constructor for a F1Racer
impl F1Racer {
    fn new() -> F1Racer {
        return F1Racer{
            name: "Max Verstapen".to_string(),
            laps: 5,
            completed_laps: 0,
            best_lap_time: 255,
            lap_times: vec![87u8, 64, 126, 95, 76],
        };
    }

    // do a lap
    fn do_lap(&mut self) {
        println!("{} is doing a new lap...", self.name);
        // remove the last lap and save it in lap times
        let lap_time = self.lap_times.pop();

        // if the option isn't empty and the best lap time is minimal then update it
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }

        // update the completed laps
        self.completed_laps += 1;
    }
}

// an implementation for an F1Racer
impl std::future::Future for F1Racer {
    // the type that the future holds
    type Output = u8;

    // periodically polls the task
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // print the thread I am in
        println!("Thread assigned is ID: {:?}", std::thread::current().id());

        // print how many laps I have completed
        if self.completed_laps < self.laps {
            self.get_mut().do_lap();
            // wakes up the future to do more work. IF YOU DO NOT DO THIS THE FUNCTION WILL HANG
            // FOREVER!!!
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }

        println!("{} has completed all laps!", self.name);
        println!("Best lap time for {} was {}", self.name, self.best_lap_time);

        return std::task::Poll::Ready(self.best_lap_time);
    }
}
