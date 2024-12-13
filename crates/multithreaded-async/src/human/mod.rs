pub trait EchoPerson {
    type Output;
    fn echo(&self) -> Self::Output;
}

pub struct Man<'a> {
    pub name: &'a str
}
pub struct Female<'a> {
    pub name: &'a str
}

impl<'a> EchoPerson for Man<'a> {
    type Output = &'a str;

    fn echo(&self) -> Self::Output {
        println!("{}", self.name);
        self.name
    }
}

impl<'a> EchoPerson for Female<'a> {
    type Output = ();

    fn echo(&self) {
        println!("{}", self.name);
    }
}