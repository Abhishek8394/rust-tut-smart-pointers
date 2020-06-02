pub struct ScopeBanner{
    msg: String,
}

impl ScopeBanner{
    pub fn new(msg: String) -> ScopeBanner{
        println!("--- ENTER SCOPE: {} ---", msg);
        ScopeBanner{
            msg: msg,
        }
    }
}

impl Drop for ScopeBanner{
    fn drop(&mut self){
        println!("--- EXIT SCOPE: {} ---", self.msg);
    }
}
