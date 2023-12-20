#[derive(Debug, Clone, PartialEq)]
pub struct Clock
{
    hour:i32,
    minutes:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let mut min = minutes;
        let mut hours = hours;
        //把分钟回正
        if min < 0 {
            while min < 0{
                min += 60;
                hours -= 1;
    
            }
        } else{
            hours += min/60;
        }

        if hours > 0 {
            //超过一天的处理一天
            while hours >= 24{
                hours -= 24;
            }
        } else{
            while hours <0{
                hours += 24;
            }
        }
        Self{
            hour: hours,
            minutes: min%60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        Self::new(self.hour,self.minutes+minutes)
    }

    pub fn to_string(&self) -> String{
        format!("{:02}:{:02}",self.hour,self.minutes)
    }

}
