pub fn inspect(s: &String){

    if  s.ends_with("s"){
        println!("The string is plural.")
    }else{
        println!("The string is singular.")

    }
}


pub fn change(s: &mut String) -> &mut String{

    if s.ends_with("s"){
        return s;
    }else{
        s.push_str("s");
        return s;
    }
}

//corect way 
pub fn change_correct(s: &mut String){
    if !s.ends_with("s"){
        s.push_str("s");
    }
}

pub fn eat(s: String)-> bool{
    let consumes = s;

    if consumes.contains("a") && consumes.starts_with("b"){
         true
    }else{
         false
    }
}

pub fn bedazzle(s: &mut String)-> &mut String{
    *s = "sparkly".to_string();
    return s;

}