use std::{vec, time::Duration};

use time::{PrimitiveDateTime as DateTime, Date, Time, Month};

fn main(){}
#[cfg(test)]
mod tests {
    use time::{PrimitiveDateTime as DateTime, Date};
    #[test]
    fn date() {
        fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
            use time::{Date, Time};
        
            DateTime::new(
                Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
                Time::from_hms(hour, minute, second).unwrap(),
            )
        }
        let start_date = dt(2015, 1, 24, 22, 0, 0);
        use crate::after;
        assert_eq!(after(start_date), dt(2046, 10, 2, 23, 46, 40));
    }
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    let gigasecond: i32 = 1000000000;
    // let year_day = 365;
    // let year_day_leap = 366;
    // let giga_second_day: i32 = 3600*24;
    // let giga_year_intger: i32 = 31;//提前计算
    // let start_year: i32 = start.year();
    // let mut current_year: i32 = start_year;
    // let mut second_count: i32 = 0;
    // let mut i: i32 = 0;
    // while i < giga_year_intger {
    //     if is_leap_year(current_year){
    //         second_count += year_day_leap*giga_second_day;
    //     }else{
    //         second_count += year_day*giga_second_day;
    //     }
    //     current_year += 1;//滞后执行？
    //     i += 1;

    // }
    // //这里处理start月、日、时、分、秒的逻辑，超过12个月要➕1年
    // let giga_second_left: i32 = gigasecond - second_count + how_many_seconds(is_leap_year(start_year), start.month(), start.day(), start.hour(), start.minute(), start.second());
    // let current_date: DateTime =  what_is_now(current_year, giga_second_left);
    // current_date

    //solution2
    start + time::Duration::seconds(gigasecond as i64)

}

fn is_leap_year(year:i32) -> bool{
    if year % 4 == 0{
        if year % 100 ==0 && year % 400 != 0{

            return false;
        }
        return true;
    }
    return false

}

fn how_many_seconds(leap_year: bool, month: time::Month, day: u8, hour: u8, minute: u8, second: u8) -> i32{
    use time::{PrimitiveDateTime as DateTime, Date};
    let mut seconds_count: i32 = 0;
    let seconds_day: i32 = 3600*24;
    let seconds_hour: i32 = 3600;
    let seconds_min: i32 = 60;
    let month_day: Vec<i32> = vec![31,59,90,120,151,181,212,243,273,304,334,365];
    //每四年一次闰年；能被100整除但不能被400整除的不是闰年
    // let giga_second_month_r: Vec<i32> = vec![31,29,31,30,31,30,31,31,30,31,30,31];
    match month{
        time::Month::January =>{}
        time::Month::February => {seconds_count = month_day[0] * seconds_day}
        time::Month::March => {seconds_count = month_day[1] * seconds_day}
        time::Month::April => {seconds_count = month_day[2] * seconds_day}
        time::Month::May => {seconds_count = month_day[3] * seconds_day}        
        time::Month::June => {seconds_count = month_day[4] * seconds_day}
        time::Month::July => {seconds_count = month_day[5] * seconds_day}
        time::Month::August => {seconds_count = month_day[6] * seconds_day}
        time::Month::September => {seconds_count = month_day[7] * seconds_day}
        time::Month::October => {seconds_count = month_day[8] * seconds_day}        
        time::Month::November => {seconds_count = month_day[9] * seconds_day}
        time::Month::December => {seconds_count = month_day[10] * seconds_day}
    }
    //如果是闰月，➕1天
    if leap_year && month != time::Month::January && month != time::Month::February{
        seconds_count += seconds_day;
    }
    //day
    seconds_count += seconds_day * (day as i32 -1);
    //hour
    seconds_count += seconds_hour * hour as i32;
    //min
    seconds_count += seconds_min * minute as i32;
    //seconds
    seconds_count += second as i32;

    return seconds_count;
}

fn what_is_now(mut year: i32,mut seconds: i32) -> DateTime{
    let seconds_min: i32 = 60;
    let seconds_hour: i32 = 3600;
    let seconds_day: i32 = 3600*24;
    let month_day: Vec<i32> = vec![31,59,90,120,151,181,212,243,273,304,334,365];                         
    let seconds_year: i32 = if is_leap_year(year) {366*seconds_day} else {365*seconds_day};
    //deal with year
    if seconds >= seconds_year {
        year += 1;
        seconds -= seconds_year;

    };
    //deal with month
    let mut month: Month = Month::January;
    if seconds < month_day[0]*seconds_day{
    }else if seconds < month_day[1]*seconds_day{
        month = Month::February;
        seconds -= month_day[0]*seconds_day;

    }else if seconds < month_day[2]*seconds_day{
        month = Month::March;
        seconds -= month_day[1]*seconds_day;

    }else if seconds < month_day[3]*seconds_day{
        month = Month::April;
        seconds -= month_day[2]*seconds_day;

    }else if seconds < month_day[4]*seconds_day{
        month = Month::May;
        seconds -= month_day[3]*seconds_day;

    }else if seconds < month_day[5]*seconds_day{
        month = Month::June;
        seconds -= month_day[4]*seconds_day;

    }else if seconds < month_day[6]*seconds_day{
        month = Month::July;
        seconds -= month_day[5]*seconds_day;

    }else if seconds < month_day[7]*seconds_day{
        month = Month::August;
        seconds -= month_day[6]*seconds_day;

    }else if seconds < month_day[8]*seconds_day{
        month = Month::September;
        seconds -= month_day[7]*seconds_day;

    }else if seconds < month_day[9]*seconds_day{
        month = Month::October;
        seconds -= month_day[8]*seconds_day;

    }else if seconds < month_day[10]*seconds_day{
        month = Month::November;
        seconds -= month_day[9]*seconds_day;

    }else if seconds < month_day[11]*seconds_day{
        month = Month::December;
        seconds -= month_day[10]*seconds_day;

    };
    //deal with day
    let mut day: i32 = seconds/seconds_day;
    seconds -= day*seconds_day;
    day += 1;


    //deal with hour
    let hour = seconds/seconds_hour;
    seconds -= hour*seconds_hour;

    //deal with min
    let minute = seconds/seconds_min;
    seconds -= minute*seconds_min;


    let date = DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day as u8).unwrap(),
        Time::from_hms(hour as u8, minute as u8, seconds as u8).unwrap(),
    );
    date
}
