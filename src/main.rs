use core::{f32, f64};
use libm::*;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Timelike,Utc};

static DEBUGENABLED:bool = true;

/*TODO : Figure out why i added the other math functions
and figure out which one to use in my calculations that will probably help out alot .
*/

fn main (){

        let convention = "MWL";
        let timezone=3.0;
        let latitude= 31.945368;
        let longitude= 35.928371;
        let is12hr=true;
        let now = Local::now();
        let (is_pm,hour)=now.hour12();
        let duhur= duhurtime(timezone, longitude);
        let (duhurhours,duhurmins)=formattedtime(duhur,is12hr);
        let (asirhours,asirmins) = formattedtime(asrtime(duhur,latitude),is12hr);
        let (sunrisehours,sunrisemins)= formattedtime(sunrise(duhur, latitude),is12hr);
        let (sunsethours,sunsetmins) = formattedtime(sunset(duhur, latitude),is12hr);
        let midnight = ((0.5)*(sunrise(duhur, latitude)-sunset(duhur, latitude)));
        let maghrib=sunset(duhur, latitude);
        let (midnighthours,midnightmins) = formattedtime(midnight,is12hr);
        let (fajirhours,fajirmins) = formattedtime(fajrtime(duhur, latitude, convention),is12hr);
        let (ishahours,ishamins)= formattedtime(ishatime(duhur, latitude, convention, maghrib, false),is12hr);
        println!("The Current time is {:02}:{:02}:{:02} {}",
        hour,now.minute(),now.second(),if is_pm {"PM"} else {"AM"});
        println!("Date : {}/{}/{}",now.year(),now.month(),now.day());
        println!("Fajir Time: {}:{}",fajirhours,fajirmins);
        println!("Sunrise Time: {}:{}",sunrisehours,sunrisemins);
        println!("Duhur time:{}:{}",duhurhours,duhurmins);  //Long of amman: 35.928371 lat: 31.945368
        println!("Asir Time: {}:{}",asirhours,asirmins);
        println!("Maghrib Time: {}:{}",sunsethours,sunsetmins+3.0);
        println!("Isha'a Time: {}:{}",ishahours,ishamins);
        println!("Midnight Time: {}:{}",midnighthours,midnightmins); // move to another func to clean up code
       // println!("Julian date: {}",julian());
        
    
    

}

pub fn sunangle(jd:f64) -> (f64,f64,f64) {
    let d = jd - 2451545.0;
    let g = fixangle( 357.529 + 0.98560028* d );
    let q = fixangle(280.459 + 0.98564736* d);
    let L = fixangle(q + 1.915* sin1(g) + 0.020* sin1(2.0*g));
    let R = 1.00014 - 0.01671* cos1(g) - 0.00014* cos1(2.0*g);
    let e = 23.439 - 0.00000036* d;
    let RA = arctan2(cos1(e)* sin1(L), cos1(L))/ 15.0;
    let bigD = arcsin(sin1(e)* sin1(L));
    let EqT = q/15.0 -  fixhour(RA) ;
    if DEBUGENABLED { // just for debug will be removed later on.
        println!("d equals : {}",d);
        println!("g equals : {}",g);
        println!("q equals : {}",q);
        println!("L equals : {}",L);
        println!("R equals : {}",R);
        println!("e equals : {}",e);
        println!("RA equals : {}",RA);
        println!("D equals : {}",bigD);
        println!("EqT equals : {}",EqT);
    }
    return (bigD,EqT,L);
}

fn bigA(t:f64,lat:f64)->f64{ // look at this and make sure we're using correct trignometry functions 
    let(d,EqT,L)=sunangle(julian()+12.0);
    let a=1.0/15.0 * arccos((sin(arccot(t+tan(lat-d)))-sin(lat)*sin(d)/cos1(lat)*cos1(d)));
    return a;
}

fn bigT(angle:f64,lat:f64)->f64{ // look at this and make sure we're using correct trignometry functions 
    let(d,EqT,L)=sunangle(julian()+12.0);
    let t=1.0/15.0 * arccos((-sin1(angle)-sin1(lat)*sin1(d))/cos1(lat)*cos1(d));
    return t;
}

fn sunrise(duhur:f64,lat:f64)->f64{
    let t=bigT(0.833, lat);
    return duhur-t;
}
fn sunset(duhur:f64,lat:f64)->f64{
    let t=bigT(0.833, lat);
    return duhur+t;
}

fn asrtime(duhur:f64,lat:f64) -> f64 {
    let t = 1.0;
    let a = bigA(t,lat);
    return duhur+a;
}


fn duhurtime(timezone:f64,Lng:f64)-> f64 {
    let (d,EqT,L)=sunangle(julian()+12.0);
    println!("EQT === {}",EqT); // DEBUG
    let dhuhr = 12.0 + timezone - Lng/15.0 - EqT;
    return dhuhr ;

}

fn fajrtime(duhur:f64,lat:f64,selector:&str)->f64{
    let mut fajr:f64=0.0;
        match selector{
            "MWL"=> fajr=duhur - bigT(18.0, lat),
            "ISNA"=> fajr=duhur - bigT(15.0, lat),
            "EGAS"=> fajr=duhur - bigT(19.5, lat),
            "UMQURA"=> fajr=duhur - bigT(18.5, lat),
            "UISK"=> fajr=duhur - bigT(18.0, lat),
            "TEHRAN"=>fajr=duhur - bigT(17.7, lat),
            "SHIA"=> println!("Dont use my app if your a shia DOGGIE"),
            _ => println!("Error"),
        }


    return fajr;
}

fn ishatime(duhur:f64,lat:f64,selector:&str,maghrib:f64,isramadan:bool)->f64{ // sending maghrib incase UMQURA is selected will clean up later
    let mut isha:f64=0.0;
        match selector{
            "MWL"=> isha=duhur + bigT(17.0, lat),
            "ISNA"=> isha=duhur + bigT(15.0, lat),
            "EGAS"=> isha=duhur + bigT(17.5, lat),
            "UMQURA"=> isha= isha_umqura(isramadan,maghrib),
            "UISK"=> isha=duhur + bigT(18.0, lat),
            "TEHRAN"=> isha=duhur + bigT(14.0, lat),
            "SHIA"=> println!("Dont use my app if your a shia DOGGIE"),

            _ => println!("Error"),
        }

    return isha;
}

fn isha_umqura(isramadan:bool,maghrib:f64)->f64{
    let mut isha:f64=0.0;
    match isramadan{
        true=>isha= maghrib + 2.0,
        false=> isha= maghrib + 1.5,
    }
    return isha;
}

fn julian() -> f64 {

    let date = Local::now();
    let mut year = date.year();
    let mut month = date.month();
    let mut day = date.day();
    if month<=2{
        year=year-1;
        month=month+12;
    }
    let A=year/100;
    let B=2 - A +A / 4;
    let JD = ((365.25 * (year as f64+ 4716.0)) as u32+ (30.6001 * (month as f64 + 1.0)) as u32) as f64+ day as f64 + B as f64 - 1524.5;
    return JD; 
}


fn fix(a:f64,num:f64)-> f64 { 
   let mut  a = a - num * floor(a/num);
   return a ;
}


fn formattedtime(time:f64,format:bool)->(f64,f64){
    let mut timeafter = fixhour(time + 0.5 /60.0);
    let mut hours = timeafter as u32 as f64;
    let mut minutes = ((timeafter-hours)*60.0 ) as u32 as f64;
    if format && hours>12.0 {
        hours = hours-12.0;
    }

    return (hours,minutes);// need to implement adding a 0 if the minutes are less than 10. there's a function that got commented mby ill complete the implementation later.

}
/* 
fn formattedtime(time:f64,format:bool)->(String,String){ // want to add ,adjust:f64 later but for now let's forget it
    let mut timeafter = fixhour(time + 0.5 /60.0);
    let mut hours = timeafter as u32 as f64;
    let mut min = ((timeafter-hours)*60.0 ) as u32 as f64;
    let mut minutes = stringify!(min).to_owned();
    if format && hours>12.0 {
        hours = hours-12.0;
    }

    return (hours.to_string(),minutes);

}*/

// Math functions to make life easier
fn fixhour(hour:f64)->f64{return fix(hour,24.0);}
fn fixangle(angle:f64)->f64{return fix(angle,360.0);}
fn sin1(degree:f64)->f64{ return sin(radians(degree))}
fn tan1(degree:f64)->f64{ return tan(radians(degree))}
fn cos1(degree:f64)->f64{ return cos(radians(degree))}
fn radians(deg:f64)->f64{ return (deg * (3.14159265359 / 180.0));}
fn degrees(deg:f64)->f64{ return (deg * (180.0/3.14159265359));}
fn arcsin(x:f64)->f64{ return degrees(asin(x))}
fn arccos(x:f64)->f64{ return degrees(acos(x))}
fn arctan(x:f64)->f64{ return degrees(atan(x))}
fn arctan2(y:f64,x:f64)->f64{ return degrees(atan2(y,x))}
fn arccot(x:f64)->f64{return 3.14159265359/2.0 - arctan(x)}



