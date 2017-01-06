use specs;
use components::*;
use specs::Join;
use super::*;

pub struct WeaponSystem;
impl specs::System<::utils::UpdateContext> for WeaponSystem {
    fn run(&mut self, arg: specs::RunArg, context: ::utils::UpdateContext) {
        let (mut weapons, mut next_weapons, shoots, entities) = arg.fetch(|world| {
            (
                world.write::<Weapon>(),
                world.write::<NextWeapon>(),
                world.read::<Shoot>(),
                world.entities(),
            )
        });

        // create a fake weapon if next weapon and none weapon
        for (_, entity) in (&mut next_weapons, &entities).iter() {
            if weapons.get(entity).is_none() {
                let fake_weapon =  Weapon {
                    reload_factor: 0.,
                    setup_factor: 0.,
                    setdown_factor: 0.,
                    state: State::Setdown(1.),
                    kind: Kind::Sniper,
                };
                match weapons.insert(entity, fake_weapon) {
                    specs::InsertResult::Inserted => (),
                    _ => unreachable!(),
                }
            }
        }

        for (weapon, entity) in (&mut weapons, &entities).iter() {
            let shoot = shoots.get(entity).is_some();

            // set down
            match weapon.state {
                State::Setdown(_) => (),
                ref mut state @ _ => if next_weapons.get(entity).is_some() {
                    *state = State::Setdown(0.);
                },
            }

            // update loading
            match weapon.state {
                State::Reload(ref mut t) => *t += context.dt*weapon.reload_factor,
                State::Setup(ref mut t) => *t += context.dt*weapon.setup_factor,
                State::Setdown(ref mut t) => *t += context.dt*weapon.setdown_factor,
                State::Ready => (),
            }

            // update state and weapon if set down
            match weapon.state {
                State::Reload(t) | State::Setup(t) => if t >= 1. {
                    if shoot {
                        weapon.state = State::Reload(t-1.);
                        if let Kind::Hammer(ref mut b) = weapon.kind {
                            *b = !*b;
                        }
                        // TODO shoot
                    } else {
                        weapon.state = State::Ready;
                    };
                },
                State::Ready => if shoot {
                    weapon.state = State::Reload(context.dt*weapon.reload_factor);
                },
                State::Setdown(t) => if t >= 1. {
                    *weapon = next_weapons.remove(entity).unwrap().0;
                    weapon.state = State::Setup(t-1.);
                },
            };
        }
    }
}
