
//Day 1 part 1 Solution
/* use ndarray::{arr1, Array1};

fn main() {
    let module_mass = arr1(&[143366,140649,64402,118831,76720,105690,68872,148364,111123,140366,105116,106380,99652,130407,68301,112756,142857,112879,52610,106354,66057,91038,120227,140647,92586,51719,72209,89889,114041,107964,71319,53380,71847,69679,117732,73292,91021,72955,105172,50072,102110,138680,131054,135512,63784,148675,113290,58014,52405,115305,87654,127350,119585,122089,52428,131275,70782,148645,66124,66831,81547,85435,134481,102166,120218,140673,84889,86363,71518,143509,80705,75446,72286,130977,80386,121639,54013,149257,73345,143555,95205,107501,97520,109658,85991,50993,54642,92644,96798,102846,125411,93821,78950,94047,55698,63822,147460,121708,139290,117748]);
    //println!("{:?}", vector);
    let fuel_mass: Array1<_> = (module_mass / 3) - 2;
    //println!("{}", new_vector);

    let total_fuel_mass: u32 = fuel_mass.iter().sum();
    println!("the total fuel mass is: {}", total_fuel_mass);
}  */

// Day 1 part 2 Solution
fn main() {
    let mut module_mass = vec![143366,140649,64402,118831,76720,105690,68872,148364,111123,140366,105116,106380,99652,130407,68301,112756,142857,112879,52610,106354,66057,91038,120227,140647,92586,51719,72209,89889,114041,107964,71319,53380,71847,69679,117732,73292,91021,72955,105172,50072,102110,138680,131054,135512,63784,148675,113290,58014,52405,115305,87654,127350,119585,122089,52428,131275,70782,148645,66124,66831,81547,85435,134481,102166,120218,140673,84889,86363,71518,143509,80705,75446,72286,130977,80386,121639,54013,149257,73345,143555,95205,107501,97520,109658,85991,50993,54642,92644,96798,102846,125411,93821,78950,94047,55698,63822,147460,121708,139290,117748];

    for module_mass in module_mass.iter_mut() {
        let mut done = false; // mut done: bool
        let mut total_fuel_mass_per_module = 0; //

        while !done {
            *module_mass = (*module_mass / 3) - 2;

            let temp = *module_mass;
            //println!("fuel_mass_per_module: {}", temp);

            total_fuel_mass_per_module = total_fuel_mass_per_module + temp;
            //println!("Total Fuel Mass Per Module:{}", total_fuel_mass_per_module);

            if temp <= 0 {
                total_fuel_mass_per_module = total_fuel_mass_per_module - temp;
                done = true;
            }
        }
        *module_mass = total_fuel_mass_per_module;
        //println!("total_fuel_mass_per_module: {}", module_mass);
    }
    //println!("module_fuel_mass vector: {:?}", module_mass);

    let total_module_fuel_mass: i32 = module_mass.iter().sum();
    println!("the total module fuel mass is: {}", total_module_fuel_mass);
}