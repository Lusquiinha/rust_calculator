use std::error::Error;
use log::error;

#[derive(Debug)]
pub struct ULA{
    pub visor: String,
    cache: String,
    queued_operation: Option<Operations>,
    erase: bool,
    error: bool,
}
#[derive(Debug, Clone)]
pub enum Operations {
    Percentage,
    Multiplication,
    Division,
    Addition,
    Subtraction,
    Equals,
    Point
}

impl ULA {
    pub fn erase_visor(&mut self) {
        self.visor = "0".to_string();
    }
    pub fn percentage(&mut self) {
        println!("Não implementado");
    }
    pub fn erase_all(&mut self){
        self.erase_visor();
        self.cache = "0".to_string();
        self.queued_operation = None;
        self.erase = false;
        self.error = false;
    }
    pub fn point(&mut self){
        println!("Não implementado");
    }
    fn do_operation(&mut self, operation: Operations){
        match self.do_queued_operation() {
            Ok(_) => {
                self.queued_operation = Some(operation.clone());
            }
            Err(_) => {
                self.erase_all();
                self.visor = "Error".to_string();
                self.erase = true;
                self.error = true;
            }
        }
    }
    pub fn queue_operation(&mut self, operation: Operations){
        println!("{:?}", self);

        if self.error{
            return;
        }

        if matches!(operation, Operations::Equals){
            self.do_operation(operation);
            return
        }

        match &self.queued_operation {
            None => {
                self.cache = self.visor.clone();
                self.queued_operation = Some(operation.clone());
                self.erase = true;
            },
            Some(_) => {
                self.do_operation(operation);
            }
        }
    }
    fn check_sum(&mut self, sum: Option<i64>)->Result<(),()>{
        match sum {
            Some(sum) => {
                self.cache = format!("{}", sum);
                self.visor = format!("{}", sum);
                self.erase = true;
                self.queued_operation = None;
                Ok(())
            }
            None => {
                Err(())
            }
        }
    }
    pub fn do_queued_operation(&mut self) -> Result<(),()>{
        match &self.queued_operation {
            Some(Operations::Addition) => {
                let sum = self.cache.parse::<i64>().unwrap().checked_add(self.visor.parse::<i64>().unwrap());
                self.check_sum(sum)
            }
            Some(Operations::Subtraction) => {
                let sum= self.cache.parse::<i64>().unwrap().checked_sub(self.visor.parse::<i64>().unwrap());
                self.check_sum(sum)
            }
            Some(Operations::Division) => {
                let sum = self.cache.parse::<i64>().unwrap().checked_div(self.visor.parse::<i64>().unwrap());
                self.check_sum(sum)
            }
            Some(Operations::Multiplication) => {
                let sum = self.cache.parse::<i64>().unwrap().checked_mul(self.visor.parse::<i64>().unwrap());
                self.check_sum(sum)
            }

            _ =>{
                Err(())
            }
        }

    }
    pub fn new() -> Self {
        Self {
            visor: String::from("0"),
            cache: String::new(),
            erase: false,
            queued_operation: None,
            error: false,
        }
    }
    pub fn concat_number(&mut self, number: u32){
        if self.error {
            self.error = false;
        }
        if self.erase{
            self.erase_visor();
            self.erase = false;
        }
        if self.visor.len() < 13{
            if self.visor == "0" {
                self.visor.pop();
            }
            self.visor.push(char::from_digit(number, 10).unwrap());
            // self.visor = "lucas".to_string();
        }
    }
    pub fn backspace(&mut self){
        if self.error {
            self.error = false;
        }
        if self.erase{
            self.erase_visor();
            self.erase = false;
            return;
        }
        self.visor.pop();
        if self.visor.is_empty(){
            self.visor.push('0');
        }
    }


}