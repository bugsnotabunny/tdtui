use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

#[proc_macro_derive(TowerConstructor)]
pub fn derive_tower_construcror(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(position: Point) -> Self {
                Self {
                    aim: Aim::new(None),
                    position: position,
                    cooldown_clock: Clock::from_now(),
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Tower)]
pub fn derive_tower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Tower for #name #ty_generics #where_clause {
            fn position(&self) -> &Point {
                &self.position
            }

            fn cost(&self) -> u64 {
                Self::COST
            }

            fn range(&self) -> f32 {
                Self::RANGE
            }
        }

        impl #impl_generics UpdatableObject for #name #ty_generics #where_clause  {
            fn on_update(&mut self, game_model: &mut dyn GameModel, _: Duration) {
                self.update_aim(game_model);
                if self.cooldown_clock.elapsed() > Self::COOLDOWN {
                    self.shoot(game_model);
                    self.cooldown_clock.tick();
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            fn shoot(&mut self, game_model: &mut dyn GameModel) {
                self.aim.try_shoot(Self::DAMAGE, |reward| {
                    game_model.wallet_mut().add_money(reward);
                });
            }

            fn update_aim(&mut self, game_model: &dyn GameModel) {
                if !self.aim.is_in_shoot_range(self, game_model.trajectory())
                    || !self.aim.is_alive()
                {
                    self.aim = Aim::new(None);
                }

                if self.aim.is_some() {
                    return;
                }

                let random_chosen_enemy = game_model
                    .enemies()
                    .iter()
                    .filter(|enemy| {
                        let enemypos = game_model.trajectory().get_point(enemy.borrow().position());
                        enemypos.distance(self.position()) < self.range()
                    })
                    .map(|rc| rc.clone())
                    .choose(&mut rand::thread_rng());

                self.aim = Aim::new(random_chosen_enemy);
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnemyConstructor)]
pub fn derive_enemy_constructor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(health: f32, speed: f32, position: f32, reward: u64) -> Self {
                Self {
                    health: health,
                    speed: speed,
                    position: position,
                    reward: reward,
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnemyConstructorResist)]
pub fn derive_enemy_constructor_resist(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(health: f32, speed: f32, position: f32, resist: f32, reward: u64) -> Self {
                Self {
                    health: health,
                    speed: speed,
                    position: position,
                    reward: reward,
                    resist: resist
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnemyCommon)]
pub fn derive_enemy_common(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics UpdatableObject for #name #ty_generics #where_clause {
            fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
                self.move_forward(delta_time, game_model.trajectory());
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            fn move_forward(
                &mut self,
                delta_time: Duration,
                trajectory: &dyn Trajectory,
            ) {
                const INITIAL_STEP: f32 = 1e-3;
                const EPSILON_MULTIPLYER: f32 = 1e2;
                const EPSILON: f32 = f32::EPSILON * EPSILON_MULTIPLYER;

                let mut move_points = self.speed * delta_time.as_secs_f32();
                let mut step = INITIAL_STEP;
                let mut position = self.position;
                while move_points > EPSILON {
                    let t_to_move_to = position + step;
                    let self_pos = trajectory.get_point(position);
                    let point_to_move_to = trajectory.get_point(t_to_move_to);
                    let distance = self_pos.distance(&point_to_move_to);

                    if distance > move_points {
                        step /= 2.0;
                        continue;
                    }

                    move_points -= step;
                    position += step;
                }
                self.position = position;
            }
        }


        impl #impl_generics EnemyCommon for #name #ty_generics #where_clause {
            fn position(&self) -> f32 {
                self.position
            }

            fn is_dead(&self) -> bool {
                self.health <= 0.0
            }

            fn reward(&self) -> u64 {
                self.reward
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
