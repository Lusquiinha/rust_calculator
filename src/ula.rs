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
    pub fn invert(&mut self) {
        if self.error {
            self.error = false;
        }
        if self.visor == "0"{
            return;
        }
        if self.visor.starts_with('-'){
            self.visor.remove(0);
        } else {
            self.visor.insert(0, '-');
        }
        if self.erase{
            self.cache = self.visor.clone();
        }

    }
    pub fn erase_all(&mut self){
        self.erase_visor();
        self.cache = "0".to_string();
        self.queued_operation = None;
        self.erase = false;
        self.error = false;
    }
    pub fn point(&mut self){
        if self.error {
            self.error = false;
        }
        if self.erase{
            self.erase_visor();
            self.erase = false;
        }
        if !self.visor.contains('.') {
            self.visor.push('.');
        }

    }
    fn do_operation(&mut self, operation: Option<Operations>){
        match self.do_queued_operation() {
            Ok(_) => {
                self.queued_operation = operation.clone();
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

        if self.error{
            return;
        }

        if matches!(operation, Operations::Equals){
            self.do_operation(None);
            return;
        }

        match &self.queued_operation {
            None => {
                self.cache = self.visor.clone();
                self.queued_operation = Some(operation.clone());
                self.erase = true;
            },
            Some(_) => {
                self.do_operation(Some(operation.clone()));
            }
        }
        println!("{:?}", self);
    }

    fn format_sum(&mut self, sum: f64) -> String {
        let formatted_sum = format!("{:15}", sum).trim().to_string();

        if formatted_sum.len() > 17 {
            let formatted_sum = format!("{:.11e}", sum).trim().to_string();
            formatted_sum
        } else {
            formatted_sum
        }
    }
    fn check_sum_float(&mut self, sum: Option<f64>)->Result<(),()>{
        match sum {
            Some(sum) => {
                let formatted_sum = self.format_sum(sum);
                self.cache = formatted_sum.clone();
                self.visor = formatted_sum;
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
                let sum = self.cache.parse::<f64>().unwrap() + self.visor.parse::<f64>().unwrap();
                self.check_sum_float(Some(sum))
            }
            Some(Operations::Subtraction) => {
                let sum= self.cache.parse::<f64>().unwrap() - self.visor.parse::<f64>().unwrap();
                self.check_sum_float(Some(sum))
            }
            Some(Operations::Division) => {
                let sum = self.cache.parse::<f64>().unwrap() / self.visor.parse::<f64>().unwrap();
                self.check_sum_float(Some(sum))
            }
            Some(Operations::Multiplication) => {
                let sum = self.cache.parse::<f64>().unwrap() * self.visor.parse::<f64>().unwrap();
                self.check_sum_float(Some(sum))
            }

            _ =>{
                Ok(())
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
        if self.visor.len() < 17{
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