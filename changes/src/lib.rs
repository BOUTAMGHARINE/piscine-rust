#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8){

	let indice = find(lights: &mut [Light], alias: &str)

	lights[indice].brightness =value;
}


pub fn find(lights: &mut [Light],alias: &str)usize {


	for (i,v) in lights.iter().enumerate(){

		if v.alias == alias {
			return i
		}
	}
	return -1




}