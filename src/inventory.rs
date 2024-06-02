use serde::{Deserialize, Serialize};

macro_rules! inventory_struct {
    (@start $field:ident, $($fields:ident),*) => {
        inventory_struct!(@acc $($fields),* -> $field: u8);
    };
    (@acc $field:ident -> $($acc:tt)*) => {
        inventory_struct!(@finish $($acc)*, $field: u8);
    };
    (@acc $field:ident, $($fields:ident),* -> $($acc:tt)*) => {
        inventory_struct!(@acc $($fields),* -> $($acc)*, $field: u8);
    };
    (@finish $($acc:tt)*) => {
        #[derive(Serialize, Deserialize)]
        pub struct Inventory {
            $($acc)*
        }
    }
}

macro_rules! inventory_from_vec {
    (@start $field:ident, $($fields:ident),*) => {
        impl From<Vec<u8>> for Inventory {
            fn from(mut v: Vec<u8>) -> Inventory {
                v.reverse();
                inventory_from_vec!(@acc v $($fields),* -> $field: v.pop().unwrap_or(0))
            }
        }
    };
    (@acc $v:ident $field:ident -> $($acc:tt)*) => {
        inventory_from_vec!(@finish $($acc)*, $field: $v.pop().unwrap_or(0))
    };
    (@acc $v:ident $field:ident, $($fields:ident),* -> $($acc:tt)*) => {
        inventory_from_vec!(@acc $v $($fields),* -> $($acc)*, $field: $v.pop().unwrap_or(0))
    };
    (@finish $($acc:tt)*) => {
        Inventory {
            $($acc)*
        }
    }
}

macro_rules! inventory_into_vec {
    (@start $field:ident, $($fields:ident),*) => {
        impl Inventory {
            pub fn into_vec(self) -> Vec<u8> {
                inventory_into_vec!(@acc self $($fields),* -> self.$field)
            }
        }
    };
    (@acc $self:ident $field:ident -> $($acc:tt)*) => {
        inventory_into_vec!(@finish $($acc)*, $self.$field)
    };
    (@acc $self:ident $field:ident, $($fields:ident),* -> $($acc:tt)*) => {
        inventory_into_vec!(@acc $self $($fields),* -> $($acc)*, $self.$field)
    };
    (@finish $($acc:tt)*) => {
        vec![$($acc)*]
    };
}

macro_rules! inventory {
    ($($fields:ident),*$(,)?) => {
        inventory_struct!(@start $($fields),*);
        inventory_from_vec!(@start $($fields),*);
        inventory_into_vec!(@start $($fields),*);
    };
}

inventory! {
    hearts,
    swords,
    gold_keys,
    silver_keys,
    red_keys,
    blue_keys,
    green_keys,
    total_swords,
    total_gold_keys,
    total_silver_keys,
    total_red_keys,
    total_blue_keys,
    total_green_keys,
    portal_stones,
    gems,
    treasures,
    total_portal_stones,
    total_gems,
    total_treasures,
    hammers,
    boots,
    compasses,
    castle_puzzle,
    castle_entered,
    green_key,
    bloodmoon_effect,
    bloodmoon_count,
    bloodmoon_orb_hide,
    boss_reached,
    castle_labyrinth_open,
    swamp_secret,
    back_door_lock_1,
    back_door_lock_2,
    victory_road_solved,
    dragon_treasure,
    secret_sockets,
    secret_tokens,
    castle_skip_primed,
    castle_puzzle_solved,
    warp_portal_01,
    warp_portal_02,
    warp_portal_03,
    warp_portal_04,
    warp_portal_05,
    warp_portal_06,
    warp_portal_07,
    warp_portal_08,
    warp_portal_09,
    warp_portal_10,
    ghost_ship_entered,
    hermit_sword_acquired,
    gem_sword,
    gold_sword,
    total_steps,
    witch_cloak,
    lava_charms,
    gold_sword_door,
    spectacles,
    skeleton_key,
    water_ring,
    shield,
    gem_shield,
    stopwatch,
    snake_boss_defeated,
    fairyland_lock_1,
    fairyland_lock_2,
    fairyland_lock_3,
    fairyland_lock_4,
    fairyland_lock_5,
    fairyland_lock_6,
    fairyland_lock_7,
    fairyland_lock_8,
    fairyland_locks,
    gem_heart,
    save_count,
    dragon_killed,
    backup_swords,
    backup_gem_sword,
    backup_gem_shield,
    backup_skeleton_key,
    backup_lava_charm,
    backup_water_ring,
    backup_boots,
    backup_spectacles,
    backup_gold_sword,
    purple_keys,
    teal_keys,
    witch_health,
    backup_hammer,
    gem_boots,
    backup_compass,
    greenfight_lock_1,
    greenfight_lock_2,
    greenfight_lock_3,
    greenfight_lock_4,
    greenfight_lock_5,
    greenfight_lock_6,
    greenfight_lock_7,
    greenfight_lock_8,
    backup_red_key,
    backup_green_key,
    backup_blue_key,
    backup_shield,
    backup_treasures,
    backup_gems,
    backup_portal_stones,
    backup_silver_keys,
    backup_gold_keys,
    backup_t_treasures,
    backup_t_gems,
    backup_t_portal_stones,
    backup_t_silver_keys,
    backup_t_gold_keys,
    total_gold_doors,
    total_silver_doors,
    total_teal_doors,
    total_purple_doors,
    total_red_doors,
    total_blue_doors,
    total_green_doors,
    total_tokens,
    total_teal_keys,
    total_purple_keys,
    witch_phase1,
    witch_phase2,
    witch_phase3,
    bunny_crime_scene,
    backup_gem_boots,
    overkill,
    backdoor_banditry,
    dragonslain,
    witch_hammer,
    witch_water_ring,
    witch_lava_charm,
    witch_skeleton_key,
    witch_compass,
    witch_boots,
    witch_gem_sword_1,
    witch_gem_sword_2,
    witch_gem_sword_3,
    completion_swamp,
    completion_maze,
    completion_boots,
    completion_cloak,
    ngp,
    ngp_tokens,
    red_sword,
    red_shield,
    red_gear_skip,
    ngpp,
    rdragon_killed,
    total_ngp_tokens,
    prevented_night,
    convergence_key,
    possum_coins,
    total_possum_coins,
    backup_t_possum_coins,
    backup_possum_coins,
    broom,
    backup_broom,
    collector_eye,
    backup_collector_eye,
    mirror,
    backup_mirror,
    dragon_egg,
    backup_dragon_egg,
    save_crystal,
    backup_save_crystal,
    carrot,
    backup_carrot,
    green_sword,
    green_shield,
    backup_green_sword,
    backup_green_shield,
    bunny_love,
    bunny_level,
    hero_form_override,
    hero_color_override,
    bunny_color_override,
    evil_bunny_tamed,
    backup_ngp_tokens,
    backup_secret_tokens,
    backup_t_ngp_tokens,
    backup_t_secret_tokens,
    fishing_pole,
    fish,
    ngppp,
    rawr_1_map,
    rawr_1_x,
    rawr_1_y,
    rawr_2_map,
    rawr_2_x,
    rawr_2_y,
    rawr_3_map,
    rawr_3_x,
    rawr_3_y,
    rawr,
    dragon_door,
    sweet,
    sour,
    umami,
    red_boots,
    squishy_npc,
    solar_power,
    moonwell_seq,
    moonwell_0,
    moonwell_1,
    moonwell_2,
    moonwell_3,
    moonwell_4,
    moonwell_5,
    moonwell_6,
    moonwell_7,
    moonwell_8,
    moonwell_9,
    eclipsed,
    haphy_castle,
    rawr_4_map,
    rawr_4_x,
    rawr_4_y,
    rawr_5_map,
    rawr_5_x,
    rawr_5_y,
    rawr_6_map,
    rawr_6_x,
    rawr_6_y,
    ever_been_mooned,
}

impl Inventory {
    pub fn completion_column(&self) -> Vec<u8> {
        vec![
            self.bunny_crime_scene,
            self.carrot,
            self.bunny_love,
            self.ngp,
            self.ngpp,
            self.fishing_pole,
            self.fish,
            self.collector_eye,
            self.gem_sword,
            self.gem_shield,
            self.red_sword,
            self.red_shield,
            self.green_sword,
            self.green_shield,
            self.eclipsed,
            self.save_crystal,
            self.dragon_egg,
            self.mirror,
            self.broom,
            self.water_ring,
            self.skeleton_key,
            self.boots,
            self.gem_boots,
            self.spectacles,
            self.compasses,
            self.hammers,
            self.total_green_keys,
            self.total_red_keys,
            self.total_blue_keys,
            self.total_green_doors,
            self.total_red_doors,
            self.total_blue_doors,
            self.ever_been_mooned,
            self.dragon_door,
            self.umami,
            self.sweet,
            self.sour,
            self.bunny_level,
            self.warp_portal_01
                + self.warp_portal_02
                + self.warp_portal_03
                + self.warp_portal_04
                + self.warp_portal_05
                + self.warp_portal_06
                + self.warp_portal_07
                + self.warp_portal_08
                + self.warp_portal_09
                + self.warp_portal_10,
            self.fairyland_locks,
            self.greenfight_lock_1
                + self.greenfight_lock_2
                + self.greenfight_lock_3
                + self.greenfight_lock_4
                + self.greenfight_lock_5
                + self.greenfight_lock_6
                + self.greenfight_lock_7
                + self.greenfight_lock_8,
            self.secret_tokens,
            self.secret_sockets,
            self.gem_heart,
            self.witch_cloak,
            self.lava_charms,
            self.hearts,
            self.total_ngp_tokens,
            self.ngp_tokens,
            self.swords,
            self.total_gold_keys,
            self.total_gold_doors,
            self.total_silver_keys,
            self.total_silver_doors,
            self.total_portal_stones,
            self.total_gems,
            self.gems,
            self.total_treasures,
            self.dragon_treasure,
        ]
    }
}
