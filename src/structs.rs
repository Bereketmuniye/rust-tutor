enum  Flavor {
    Sparkling,
    Sweet,
    Fruity,
} 
 struct Drink {
    flavor:Flavor,
    fluid_oz: f64,

 }

 fn print_drink(drink: Drink){
    match drink.flavor {
        Flavor::Sparkling=>println!("Sparkling"),
        Flavor::Sweet=>println!("Sweet"),
        Flavor::Fruity=>println!("Fruity"),
    }
    println!("ox:{:?}",drink.fluid_oz);
 }

pub fn main(){
    let sweet =Drink{
        flavor:Flavor::Sparkling,
        fluid_oz:12.0,
    };
    print_drink(sweet);
    let fruity= Drink{
        flavor:Flavor::Fruity,
        fluid_oz:8.0,
    };
    print_drink(fruity);
}