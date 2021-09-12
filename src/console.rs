trait Console {
    fn read(&self);
    fn write(&mut self, s: &str);
}

pub struct SimpleConsole {
    graphic_pt: (usize, usize),
}

impl Console for SimpleConsole{
    fn read(&self){}
    fn write(&mut self,s:&str){
        for c in s.to_chars().iter(){
            
        }
    }
}