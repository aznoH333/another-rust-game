use std::time::Instant;

use super::performance_record::PerformanceRecord;

pub struct PerformanceMonitor{
    records: Vec<PerformanceRecord>,
    currently_recording: Option<String>,
    last_captured_time: Option<Instant>
}




impl PerformanceMonitor{
    pub fn new() -> PerformanceMonitor{
        return PerformanceMonitor{
            records: Vec::new(),
            currently_recording: None,
            last_captured_time: None
        };
    }

    pub fn record_section(&mut self, label: &str){
        if self.currently_recording.is_some(){
            self.save_record();
        }

        self.currently_recording = Some(label.to_owned());
        self.last_captured_time = Some(Instant::now());
    }

    fn save_record(&mut self){
        if self.currently_recording.is_none(){
            panic!("performance recording failed");
        }

        self.records.push(
            PerformanceRecord::new(
                self.currently_recording.clone().unwrap(), 
                self.last_captured_time.unwrap().elapsed()
            )
        );
    }

    pub fn end_recording(&mut self, print_result: bool){
        self.save_record();

        if print_result{
            println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
            println!("          Recording results          ");
            println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
            println!("");
            for iterator in 0..self.records.len() {
                let record = self.records.get(iterator).unwrap();
                println!("{}. {} \t \t {}ms", iterator, record.get_label(), record.get_elapsed_time().as_millis())
            }
            println!("");
            println!("Report end!");
            println!("");




        }
    }



    
}