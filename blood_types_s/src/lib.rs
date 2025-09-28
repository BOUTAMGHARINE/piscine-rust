#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
   pub fn can_receive_from(self, donor: Self) -> bool {
        // Compatibilité ABO
        let abo_ok = match self.antigen {
            Antigen::A => donor.antigen == Antigen::A || donor.antigen == Antigen::O,
            Antigen::B => donor.antigen == Antigen::B || donor.antigen == Antigen::O,
            Antigen::AB => true, // AB reçoit de tous
            Antigen::O => donor.antigen == Antigen::O,
        };

        // Compatibilité Rh
        let rh_ok = match (self.rh_factor, donor.rh_factor) {
            // receveur positif accepte positif et négatif
            (RhFactor::Positive, _) => true,
            // receveur négatif n'accepte que négatif
            (RhFactor::Negative, RhFactor::Negative) => true,
            (RhFactor::Negative, RhFactor::Positive) => false,
        };

        abo_ok && rh_ok
    }

    pub fn donors(self) -> Vec<Self> {
		let mut res = Vec::new();
        let antigon = vec![Antigen::A,Antigen::B,Antigen::O,Antigen::AB];
		let facthor = vec![RhFactor::Positive,RhFactor::Negative];

		for anti in antigon.iter() {
			for fac in facthor.iter() {
				let blood = BloodType {
					antigen : *anti,
					rh_factor : *fac
				};
				if self.can_receive_from(blood) {
					res.push(blood)
				}
			}
		}

		res

        
    }

    pub fn recipients(self) -> Vec<Self> {
		let mut res = Vec::new();
        let antigon = vec![Antigen::A,Antigen::B,Antigen::O,Antigen::AB];
		let facthor = vec![RhFactor::Positive,RhFactor::Negative];

		for anti in antigon.iter() {
			for fac in facthor.iter() {
				let blood = BloodType {
					rh_factor : *fac,
					antigen : *anti
				};
				if blood.can_receive_from(self) {
					res.push(blood)
				}
			}
		}

		res
        
    }
}
