pub struct Hero {

    // стати
    pub healpoint: f32,
    pub manapoint: f32,
    pub stamina: u8,

    // характерістіка
    //pub pawer: f32,
    //pub agile: f32,
    pub strength: f32,
    pub lifeforce: f32,
    //pub perception: f32,
    //pub intelligence: f32,
    //pub superperception: f32,

    // біографія
    //pub name: String,
    //pub towname: String,
    //pub sex: String,
    //pub age: u8,
    //pub totames: String,

    // навики боевие
    // ремесла
    // досягнення
}

impl Hero {
    pub fn new(strength: f32, lifeforce: f32,manapoint: f32) -> Self {
        // let speed =  
        let healpoint = strength * 2.0 + lifeforce * 1.5;
        let stamina_f32 = (healpoint / 2.0).clamp(1.0, 100.0);
        let mut stamina = stamina_f32 as u8;
        

        if stamina < 1 {
            stamina = 1;
        } else if stamina > 100 {
            stamina = 100;
        }
        //pub fn take_gamage($mut self,dmg: i16){
        //    self.healpoint = (self.healpoint - dmg).max(0.0); // hp >= 0
        //}
        //pub fn regeneration($mut self){
            //self.healpoint = self.healpoint + лайф вокус + крепость + тело + спели(навики)
        //}
        //pub fn use_spell(){

        //}

        Hero { healpoint, manapoint, stamina, strength, lifeforce }
    }
    
}
