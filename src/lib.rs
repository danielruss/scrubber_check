use std::{cmp::min, fmt::{Display,Formatter}};

#[derive(Debug)]
pub struct Scrubbed{
    original_value : String,
    scrubbed_value : String,
}

impl Scrubbed {
    fn new<T>(scrubbed:T,original:T) -> Self
    where T:Into<String>{
        Scrubbed{
            original_value: original.into(),
            scrubbed_value:scrubbed.into()
        }
    }
}

impl Display for Scrubbed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Original: '{}', Scrubbed: '{}'",
            self.original_value, self.scrubbed_value
        )
    }
}

pub fn compare<'o>(scrubbed:&'o str, original:&'o str) -> Vec<Scrubbed>{
    let mut result = Vec::new();
    let mut scrubbed_iter = scrubbed.char_indices();
    let mut original_iter = original.char_indices();

    while let (Some((scrubbed_index, scrubbed_char)), Some((original_index, original_char))) =
        (scrubbed_iter.next(), original_iter.next()){
        match (scrubbed_char,original_char) {
            (s,c) if s==c => {},
            (s,_c) if s=='[' => {
                let mut sindx = 0;
                let mut sz=s;
                while sz != ']' {
                    if let Some( (i,c) ) = scrubbed_iter.next(){
                        //println!("{}: {}",i,c);
                        sz=c;
                        sindx=i;
                    }
                }
                sindx+=1;

                // check the next 5 letter... for '['
                let n_check = min(5, scrubbed[sindx..].len());
                //println!("check {} chars ==> {}>{}<",n_check,sindx,&scrubbed[sindx..sindx+n_check]);
                if n_check == 0 {
                    result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..]) );
                    return result;
                } else {
                    let mut uindx = original_index+1;
                    //println!("{}-{}: {} ==>{}<",sindx,uindx,c,  &original[uindx..(uindx+n_check)]);
                    while &original[uindx..(uindx+n_check)] != &scrubbed[sindx..(sindx+n_check)] {
                        let opt = original_iter.next();
                        if let Some((oindex,_ochar)) = opt {
                            uindx = oindex+1;
                            //println!("{}-{}: {} ==>{}<",sindx,uindx,_ochar,&original[uindx..(uindx+n_check)]) ;
                        } else{
                            result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..]) );
                            return result
                        }
                    }
                    //println!("{}: >{}< {}: >{}<",sindx,&scrubbed[sindx..],uindx,&original[uindx..]);
                    result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..uindx]) );
                }
            },
            _ => {
                println!(".... out of alignment ... >{}<  >{}<",&scrubbed[scrubbed_index..],&original[original_index..]);
                return result;
            }
        }
    }
    
    result 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let original = "He has had both of his COVID vaccinations.";
        let scrubbed = "He has had both of his [PERSONALNAME] vaccinations.";
        let result = compare(scrubbed, original);
        assert_eq!(result.len(),1);
        assert_eq!(&result[0].scrubbed_value,"[PERSONALNAME]");
        assert_eq!(&result[0].original_value,"COVID");

        let original = "Both Dr. Rebbis and Frank were present at the meeting";
        let scrubbed = "Both [PROVIDERNAME] and [PERSONALNAME] were present at the meeting.";
        let result = compare(scrubbed, original);
        assert_eq!(result.len(),2);
        assert_eq!(&result[0].scrubbed_value,"[PROVIDERNAME]");
        assert_eq!(&result[0].original_value,"Dr. Rebbis");
        assert_eq!(&result[1].scrubbed_value,"[PERSONALNAME]");
        assert_eq!(&result[1].original_value,"Frank"); 
        println!("{:?}",result)
    }
}