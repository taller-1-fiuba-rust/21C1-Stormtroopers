use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_timestamp_now() -> String {
    let time_now =  SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let format_time = format_timestamp(time_now);
    let local = -3_i32;
    format!("{}/{:02}/{:02} {:02}:{:02}:{:02}",format_time.0,format_time.1,format_time.2,format_time.3 as i32+local,format_time.4,format_time.5)
}
fn format_timestamp(timestamp: u64) -> (u32,u32,u32,u32,u32,u32) {
    let time_now = timestamp;
    //println!("timestamp: {}",time_now);
    //let year_sec = 31556926_f64; //EPOC_UNIX //*31557600;//31536000
    let year_sec = 31536000_f64;
    let day_sec = 86400_f64;
    let hour_sec = 3600_f64;
    let min_sec = 60_f64;
    let base_time = 1970_f64;

    let ywtrunc = time_now as f64/year_sec;
    let years = (ywtrunc).trunc();
    let year = years + base_time;

    let dwtrunc = time_now.clone() as f64 - (years.clone() * year_sec.clone());
    let days = (dwtrunc/day_sec).trunc();

    let hwtrunc = time_now.clone() as f64 - (years.clone() * year_sec.clone()) - (days * day_sec.clone());
    let hour = (hwtrunc/hour_sec).trunc();

    let mwtrunc = time_now.clone() as f64 - (years.clone()  * year_sec.clone()) - (days.clone() * day_sec.clone()) - (hour * hour_sec.clone()) ;
    let min = ( mwtrunc/min_sec).trunc();

    let swtrunc = time_now.clone() as f64 - (years.clone()  * year_sec.clone()) - (days.clone() * day_sec.clone()) - (hour.clone() * hour_sec.clone()) - (min * min_sec.clone());
    let sec = swtrunc;
    //println!("years {}, without trunc {}",years,ywtrunc);
    //println!("days {}, without trunc {}",days,dwtrunc);
    //println!("hour {}, without trunc {}",hour,hwtrunc);
    //println!("min {}, without trunc {}", min,mwtrunc);
    //println!("sec {}, without trunc {}", sec,swtrunc);

    let mut days_match;
    days_match = (days + 1.0) as u32;

    let mut bis = 0;
    let count_bisiesto = count_bisiesto(year);
    //println!("count bisiesto: {}", count_bisiesto);

    days_match -= count_bisiesto;
    /* Bisiesto */
    if year%4_f64 == 0.0 || year%100_f64 == 0.0 && year%400_f64 == 0.0 && days_match > 59 {
        //  println!("Bisiesto year {}!", year);
        if days_match == 59 {
            bis += 1;
        }
        if days_match < 32 {
            days_match += 1;
        }

    }

//    println!("days_match: {}, bis: {}", days_match, bis);
    let month_days = match days_match {
        1..=31 =>    ( 1, days_match ),
        32..=59=>    ( 2, days_match - 31 + bis ),
        60..=90 =>   ( 3, days_match - 59  ),
        91..=120 =>  ( 4, days_match - 90  ),
        121..=151 => (5, days_match - 120 ),
        152..=181 => (6, days_match - 151 ),
        182..=212 => (7, days_match - 181 ),
        213..=243 => (8, days_match - 212 ),
        244..=273 => (9, days_match - 243),
        274..=304 => (10,days_match - 273),
        305..=334 => (11,days_match - 304),
        335..=365 => (12,days_match - 334),
        _ => {(0,0)}
    };

    //let dateformat = format!("{}/{}/{} {}:{}:{}",year,month_days.0,month_days.1,hour,min,sec);
    //println!("{}",dateformat);
    (year as u32,month_days.0,month_days.1 as u32,hour as u32,min as u32,sec as u32)
}

fn count_bisiesto(year: f64) -> u32 {
    let mut i = 1_u32;
    if year < 1972_f64 {
        return 0
    }
    loop {
        if year.clone() as u32 - 4_u32*i < 1972_u32 {
            break;
        }
        i+=1;
    }
    i.clone()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bisiesto_test(){
        assert!(0 == count_bisiesto(1970_f64));
        assert!(0 == count_bisiesto(1971_f64));
        assert!(1 == count_bisiesto(1972_f64));
        assert!(1 == count_bisiesto(1973_f64));
        assert!(1 == count_bisiesto(1975_f64));
        assert!(2 == count_bisiesto(1976_f64));
        assert!(6 == count_bisiesto(1992_f64));
    }

    #[test]
    fn format_date_january() {
        let init = format_timestamp(0); // 1970/1/1 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 1);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(2678399); // 1970/1/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 1);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);

        let init_bis = format_timestamp(63072000); // 1972/01/01 00:00:00
        assert!(init_bis.0 == 1972);
        assert!(init_bis.1 == 1);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(65750399); // 1972/01/31 23:59:59
        assert!(end_bis.0 == 1972);
        assert!(end_bis.1 == 1);
        assert!(end_bis.2 == 31);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);

        let init_bis = format_timestamp(820454400); // 1996/01/01 00:00:00
        assert!(init_bis.0 == 1996);
        assert!(init_bis.1 == 1);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(823132799); // 1996/01/31 23:59:59
        assert!(end_bis.0 == 1996);
        assert!(end_bis.1 == 1);
        assert!(end_bis.2 == 31);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);
    }

    #[test]
    fn format_date_febrary() {
        let init = format_timestamp(2678400);// 1970/02/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 2);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(5097599);// 1970/02/28 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 2);
        assert!(end.2 == 28);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);

        let init_bis = format_timestamp(65750400); // 1972/02/01 00:00:00
        assert!(init_bis.0 == 1972);
        assert!(init_bis.1 == 2);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(68255999); // 1972/02/29 23:59:59
        assert!(end_bis.0 == 1972);
        assert!(end_bis.1 == 2);
        assert!(end_bis.2 == 29);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);

        let init_bis = format_timestamp(823132800); // 1996/02/01 00:00:00
        assert!(init_bis.0 == 1996);
        assert!(init_bis.1 == 2);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(825638399); // 1996/02/29 23:59:59
        assert!(end_bis.0 == 1996);
        assert!(end_bis.1 == 2);
        assert!(end_bis.2 == 29);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);
    }

    #[test]
    fn format_date_march() {
        let init = format_timestamp(5097600);// 1970/03/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 3);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(7775999);// 1970/03/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 3);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);

        let init_bis = format_timestamp(68256000); // 1972/03/01 00:00:00
        assert!(init_bis.0 == 1972);
        assert!(init_bis.1 == 3);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(70934399); // 1972/03/31 23:59:59
        assert!(end_bis.0 == 1972);
        assert!(end_bis.1 == 3);
        assert!(end_bis.2 == 31);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);
    }

    #[test]
    fn format_date_april() {
        let init = format_timestamp(7776000);// 1970/04/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 4);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(10367999);// 1970/04/30 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 4);
        assert!(end.2 == 30);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_may() {
        let init = format_timestamp(10368000);// 1970/05/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 5);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(13046399);// 1970/05/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 5);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_june() {
        let init = format_timestamp(13046400);// 1970/06/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 6);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(15638399);// 1970/06/30 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 6);
        assert!(end.2 == 30);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_july() {
        let init = format_timestamp(15638400);// 1970/07/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 7);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(18316799);// 1970/07/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 7);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_august() {
        let init = format_timestamp(18316800);// 1970/08/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 8);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(20995199);// 1970/08/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 8);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);

        let init_bis = format_timestamp(81475200); // 1972/08/01 00:00:00
        assert!(init_bis.0 == 1972);
        assert!(init_bis.1 == 8);
        assert!(init_bis.2 == 1);
        assert!(init_bis.3 == 0);
        assert!(init_bis.4 == 0);
        assert!(init_bis.5 == 0);

        let end_bis = format_timestamp(84153599); // 1972/08/31 23:59:59
        assert!(end_bis.0 == 1972);
        assert!(end_bis.1 == 8);
        assert!(end_bis.2 == 31);
        assert!(end_bis.3 == 23);
        assert!(end_bis.4 == 59);
        assert!(end_bis.5 == 59);
    }

    #[test]
    fn format_date_september() {
        let init = format_timestamp(20995200);// 1970/09/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 9);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(23587199);// 1970/09/30 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 9);
        assert!(end.2 == 30);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_october() {
        let init = format_timestamp(23587200);// 1970/10/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 10);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(26265599);// 1970/10/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 10);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_november() {
        let init = format_timestamp(26265600);// 1970/11/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 11);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(28857599);// 1970/11/30 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 11);
        assert!(end.2 == 30);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }

    #[test]
    fn format_date_december() {
        let init = format_timestamp(28857600);// 1970/12/01 00:00:00
        assert!(init.0 == 1970);
        assert!(init.1 == 12);
        assert!(init.2 == 1);
        assert!(init.3 == 0);
        assert!(init.4 == 0);
        assert!(init.5 == 0);

        let end = format_timestamp(31535999);// 1970/12/31 23:59:59
        assert!(end.0 == 1970);
        assert!(end.1 == 12);
        assert!(end.2 == 31);
        assert!(end.3 == 23);
        assert!(end.4 == 59);
        assert!(end.5 == 59);
    }
}