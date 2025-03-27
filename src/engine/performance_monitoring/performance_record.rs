use std::time::Duration;

pub struct PerformanceRecord{
    label: String,
    seconds_elapsed: Duration
}


impl PerformanceRecord{
    pub fn new(label: String, seconds_elapsed: Duration) -> PerformanceRecord{
        return PerformanceRecord 
        { 
            label: label, 
            seconds_elapsed: seconds_elapsed 
        }
    }

    pub fn get_label(&self) -> &String {
        return &self.label;
    }

    pub fn get_elapsed_time(&self) -> &Duration {
        return &self.seconds_elapsed;
    }
}