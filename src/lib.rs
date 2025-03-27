use std::fmt::{Display,Formatter};
use std::cmp::min;

#[derive(Debug,Clone,PartialEq, Eq)]
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
            "Scrubbed: '{}', Original: '{}'",
            self.scrubbed_value, self.original_value
        )
    }
}

impl From<&Scrubbed> for String{
    fn from(value: &Scrubbed) -> Self {
        format!("Scrubbed: '{}', Original: '{}'",value.scrubbed_value, value.original_value)
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
            (s,c) if s=='[' => {
                // the next line shut the compiler up
                // about not using 'c' 
                _ = c.to_ascii_lowercase();
                let mut sindx = 0;
                let mut sz=s;
                let mut n_check=5usize;
                while sz != ']' {
                    if let Some( (i,c) ) = scrubbed_iter.next(){
                        //println!("{}: {}",i,c);
                        sz=c;
                        sindx=i;
                        // hit the end of the Scrubbed text...
                        if sz==']'{
                            n_check = if sindx + n_check > scrubbed.len() {scrubbed.len()-sindx-1} else { n_check };
                            let check_s = &scrubbed[sindx..sindx+n_check];
                            match check_s.find('[') {
                                // two back to back items where scrubbed...
                                // dont stop combine the two
                                Some(0) =>  sz=' ',
                                // truncate n_check
                                Some(n) => n_check=n-1,
                                // no '['
                                None => {}
                            } 
                        }
                    }
                }
                sindx+=1;

                // check the next n letter... 
                //println!("check {} chars ==> {}>{}<",n_check,sindx,&scrubbed[sindx..sindx+n_check]);
                if n_check == 0 {
                    result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..]) );
                    return result;
                } else {
                    let mut uindx = original_index+1;
                    // make sure the n_char doesn't overflow the string...
                    //println!("ncheck og: {}",n_check);
                    n_check = if uindx+n_check>=original.len() {original.len()-n_check-1} else {n_check};
                    //println!("{}-{}: {} ==>{}< {}",sindx,uindx,c,  &original[uindx..(uindx+n_check)],n_check);
                    while &original[uindx..(uindx+n_check)] != &scrubbed[sindx..(sindx+n_check)] {
                        let opt = original_iter.next();
                        if let Some((oindex,_ochar)) = opt {
                            uindx = oindex+1;
                            //println!("{}-{}: {} ==>{}<",sindx,uindx,_ochar,&original[uindx..(uindx+n_check)]) ;
                        } else{
                            result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..]) );
                            return result
                        }
                        if uindx>original_index+50{
                            return result;
                        }
                    }
                    //println!("{}: >{}< {}: >{}<",sindx,&scrubbed[sindx..],uindx,&original[uindx..]);
                    result.push(Scrubbed::new(&scrubbed[scrubbed_index..sindx],&original[original_index..uindx]) );
                }
            },
            (s,c) => {
                eprintln!("scrub char: {}\norig char: {}",s,c);
                let nchars_s=min(10,scrubbed.len()-scrubbed_index);
                let nchars_o=min(10,original.len()-original_index);
                eprintln!(".... out of alignment ... >{}<  >{}<",&scrubbed[scrubbed_index..scrubbed_index+nchars_s],&original[original_index..original_index+nchars_o]);
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
        println!("{:?}",result);
        println!("==> fmt: {}",result[0]);
        println!("==> from: {}",String::from(&result[0]));

        let original="Hi there I am Dan.";
        let scrubbed="Hi there I am [PERSON].";
        let result = compare(scrubbed, original);
        println!("{:?}",result);

    }
}