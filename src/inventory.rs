use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    hearts: u8,
    swords: u8,
    gold_keys: u8,
    silver_keys: u8,
    red_keys: u8,
    blue_keys: u8,
    green_keys: u8,
    total_swords: u8,
    total_gold_keys: u8,
    total_silver_keys: u8,
    total_red_keys: u8,
    total_blue_keys: u8,
    total_green_keys: u8,
    portal_stones: u8,
    gems: u8,
    treasures: u8,
    total_portal_stones: u8,
    total_gems: u8,
    total_treasures: u8,
    hammers: u8,
    boots: u8,
    compasses: u8,
    castle_puzzle: u8,
    castle_entered: u8,
    green_key: u8,
    bloodmoon_effect: u8,
    bloodmoon_count: u8,
    bloodmoon_orb_hide: u8,
    boss_reached: u8,
    castle_labyrinth_open: u8,
    swamp_secret: u8,
    back_door_lock_1: u8,
    back_door_lock_2: u8,
    victory_road_solved: u8,
    dragon_treasure: u8,
    secret_sockets: u8,
    secret_tokens: u8,
    castle_skip_primed: u8,
    castle_puzzle_solved: u8,
    warp_portal_01: u8,
    warp_portal_02: u8,
    warp_portal_03: u8,
    warp_portal_04: u8,
    warp_portal_05: u8,
    warp_portal_06: u8,
    warp_portal_07: u8,
    warp_portal_08: u8,
    warp_portal_09: u8,
    warp_portal_10: u8,
    ghost_ship_entered: u8,
    hermit_sword_acquired: u8,
    gem_sword: u8,
    gold_sword: u8,
    total_steps: u8,
    witch_cloak: u8,
    lava_charms: u8,
    gold_sword_door: u8,
    spectacles: u8,
    skeleton_key: u8,
    water_ring: u8,
    shield: u8,
    gem_shield: u8,
    stopwatch: u8,
    snake_boss_defeated: u8,
    fairyland_lock_1: u8,
    fairyland_lock_2: u8,
    fairyland_lock_3: u8,
    fairyland_lock_4: u8,
    fairyland_lock_5: u8,
    fairyland_lock_6: u8,
    fairyland_lock_7: u8,
    fairyland_lock_8: u8,
    fairyland_locks: u8,
    gem_heart: u8,
    save_count: u8,
    dragon_killed: u8,
    backup_swords: u8,
    backup_gem_sword: u8,
    backup_gem_shield: u8,
    backup_skeleton_key: u8,
    backup_lava_charm: u8,
    backup_water_ring: u8,
    backup_boots: u8,
    backup_spectacles: u8,
    backup_gold_sword: u8,
    purple_keys: u8,
    teal_keys: u8,
    witch_health: u8,
    backup_hammer: u8,
    gem_boots: u8,
    backup_compass: u8,
    greenfight_lock_1: u8,
    greenfight_lock_2: u8,
    greenfight_lock_3: u8,
    greenfight_lock_4: u8,
    greenfight_lock_5: u8,
    greenfight_lock_6: u8,
    greenfight_lock_7: u8,
    greenfight_lock_8: u8,
    backup_red_key: u8,
    backup_green_key: u8,
    backup_blue_key: u8,
    backup_shield: u8,
    backup_treasures: u8,
    backup_gems: u8,
    backup_portal_stones: u8,
    backup_silver_keys: u8,
    backup_gold_keys: u8,
    backup_t_treasures: u8,
    backup_t_gems: u8,
    backup_t_portal_stones: u8,
    backup_t_silver_keys: u8,
    backup_t_gold_keys: u8,
    total_gold_doors: u8,
    total_silver_doors: u8,
    total_teal_doors: u8,
    total_purple_doors: u8,
    total_red_doors: u8,
    total_blue_doors: u8,
    total_green_doors: u8,
    total_tokens: u8,
    total_teal_keys: u8,
    total_purple_keys: u8,
    witch_phase1: u8,
    witch_phase2: u8,
    witch_phase3: u8,
    bunny_crime_scene: u8,
    backup_gem_boots: u8,
    overkill: u8,
    backdoor_banditry: u8,
    dragonslain: u8,
    witch_hammer: u8,
    witch_water_ring: u8,
    witch_lava_charm: u8,
    witch_skeleton_key: u8,
    witch_compass: u8,
    witch_boots: u8,
    witch_gem_sword_1: u8,
    witch_gem_sword_2: u8,
    witch_gem_sword_3: u8,
    completion_swamp: u8,
    completion_maze: u8,
    completion_boots: u8,
    completion_cloak: u8,
    ngp: u8,
    ngp_tokens: u8,
    red_sword: u8,
    red_shield: u8,
    red_gear_skip: u8,
    ngpp: u8,
    rdragon_killed: u8,
    total_ngp_tokens: u8,
    prevented_night: u8,
    convergence_key: u8,
    possum_coins: u8,
    total_possum_coins: u8,
    backup_t_possum_coins: u8,
    backup_possum_coins: u8,
    broom: u8,
    backup_broom: u8,
    collector_eye: u8,
    backup_collector_eye: u8,
    mirror: u8,
    backup_mirror: u8,
    dragon_egg: u8,
    backup_dragon_egg: u8,
    save_crystal: u8,
    backup_save_crystal: u8,
    carrot: u8,
    backup_carrot: u8,
    green_sword: u8,
    green_shield: u8,
    backup_green_sword: u8,
    backup_green_shield: u8,
    bunny_love: u8,
    bunny_level: u8,
    hero_form_override: u8,
    hero_color_override: u8,
    bunny_color_override: u8,
    evil_bunny_tamed: u8,
    backup_ngp_tokens: u8,
    backup_secret_tokens: u8,
    backup_t_ngp_tokens: u8,
    backup_t_secret_tokens: u8,
    fishing_pole: u8,
    fish: u8,
    ngppp: u8,
    rawr_1_map: u8,
    rawr_1_x: u8,
    rawr_1_y: u8,
    rawr_2_map: u8,
    rawr_2_x: u8,
    rawr_2_y: u8,
    rawr_3_map: u8,
    rawr_3_x: u8,
    rawr_3_y: u8,
    rawr: u8,
}

impl From<Vec<u8>> for Inventory {
    fn from(mut v: Vec<u8>) -> Self {
        v.reverse();
        Inventory {
            hearts: v.pop().unwrap_or(0),
            swords: v.pop().unwrap_or(0),
            gold_keys: v.pop().unwrap_or(0),
            silver_keys: v.pop().unwrap_or(0),
            red_keys: v.pop().unwrap_or(0),
            blue_keys: v.pop().unwrap_or(0),
            green_keys: v.pop().unwrap_or(0),
            total_swords: v.pop().unwrap_or(0),
            total_gold_keys: v.pop().unwrap_or(0),
            total_silver_keys: v.pop().unwrap_or(0),
            total_red_keys: v.pop().unwrap_or(0),
            total_blue_keys: v.pop().unwrap_or(0),
            total_green_keys: v.pop().unwrap_or(0),
            portal_stones: v.pop().unwrap_or(0),
            gems: v.pop().unwrap_or(0),
            treasures: v.pop().unwrap_or(0),
            total_portal_stones: v.pop().unwrap_or(0),
            total_gems: v.pop().unwrap_or(0),
            total_treasures: v.pop().unwrap_or(0),
            hammers: v.pop().unwrap_or(0),
            boots: v.pop().unwrap_or(0),
            compasses: v.pop().unwrap_or(0),
            castle_puzzle: v.pop().unwrap_or(0),
            castle_entered: v.pop().unwrap_or(0),
            green_key: v.pop().unwrap_or(0),
            bloodmoon_effect: v.pop().unwrap_or(0),
            bloodmoon_count: v.pop().unwrap_or(0),
            bloodmoon_orb_hide: v.pop().unwrap_or(0),
            boss_reached: v.pop().unwrap_or(0),
            castle_labyrinth_open: v.pop().unwrap_or(0),
            swamp_secret: v.pop().unwrap_or(0),
            back_door_lock_1: v.pop().unwrap_or(0),
            back_door_lock_2: v.pop().unwrap_or(0),
            victory_road_solved: v.pop().unwrap_or(0),
            dragon_treasure: v.pop().unwrap_or(0),
            secret_sockets: v.pop().unwrap_or(0),
            secret_tokens: v.pop().unwrap_or(0),
            castle_skip_primed: v.pop().unwrap_or(0),
            castle_puzzle_solved: v.pop().unwrap_or(0),
            warp_portal_01: v.pop().unwrap_or(0),
            warp_portal_02: v.pop().unwrap_or(0),
            warp_portal_03: v.pop().unwrap_or(0),
            warp_portal_04: v.pop().unwrap_or(0),
            warp_portal_05: v.pop().unwrap_or(0),
            warp_portal_06: v.pop().unwrap_or(0),
            warp_portal_07: v.pop().unwrap_or(0),
            warp_portal_08: v.pop().unwrap_or(0),
            warp_portal_09: v.pop().unwrap_or(0),
            warp_portal_10: v.pop().unwrap_or(0),
            ghost_ship_entered: v.pop().unwrap_or(0),
            hermit_sword_acquired: v.pop().unwrap_or(0),
            gem_sword: v.pop().unwrap_or(0),
            gold_sword: v.pop().unwrap_or(0),
            total_steps: v.pop().unwrap_or(0),
            witch_cloak: v.pop().unwrap_or(0),
            lava_charms: v.pop().unwrap_or(0),
            gold_sword_door: v.pop().unwrap_or(0),
            spectacles: v.pop().unwrap_or(0),
            skeleton_key: v.pop().unwrap_or(0),
            water_ring: v.pop().unwrap_or(0),
            shield: v.pop().unwrap_or(0),
            gem_shield: v.pop().unwrap_or(0),
            stopwatch: v.pop().unwrap_or(0),
            snake_boss_defeated: v.pop().unwrap_or(0),
            fairyland_lock_1: v.pop().unwrap_or(0),
            fairyland_lock_2: v.pop().unwrap_or(0),
            fairyland_lock_3: v.pop().unwrap_or(0),
            fairyland_lock_4: v.pop().unwrap_or(0),
            fairyland_lock_5: v.pop().unwrap_or(0),
            fairyland_lock_6: v.pop().unwrap_or(0),
            fairyland_lock_7: v.pop().unwrap_or(0),
            fairyland_lock_8: v.pop().unwrap_or(0),
            fairyland_locks: v.pop().unwrap_or(0),
            gem_heart: v.pop().unwrap_or(0),
            save_count: v.pop().unwrap_or(0),
            dragon_killed: v.pop().unwrap_or(0),
            backup_swords: v.pop().unwrap_or(0),
            backup_gem_sword: v.pop().unwrap_or(0),
            backup_gem_shield: v.pop().unwrap_or(0),
            backup_skeleton_key: v.pop().unwrap_or(0),
            backup_lava_charm: v.pop().unwrap_or(0),
            backup_water_ring: v.pop().unwrap_or(0),
            backup_boots: v.pop().unwrap_or(0),
            backup_spectacles: v.pop().unwrap_or(0),
            backup_gold_sword: v.pop().unwrap_or(0),
            purple_keys: v.pop().unwrap_or(0),
            teal_keys: v.pop().unwrap_or(0),
            witch_health: v.pop().unwrap_or(0),
            backup_hammer: v.pop().unwrap_or(0),
            gem_boots: v.pop().unwrap_or(0),
            backup_compass: v.pop().unwrap_or(0),
            greenfight_lock_1: v.pop().unwrap_or(0),
            greenfight_lock_2: v.pop().unwrap_or(0),
            greenfight_lock_3: v.pop().unwrap_or(0),
            greenfight_lock_4: v.pop().unwrap_or(0),
            greenfight_lock_5: v.pop().unwrap_or(0),
            greenfight_lock_6: v.pop().unwrap_or(0),
            greenfight_lock_7: v.pop().unwrap_or(0),
            greenfight_lock_8: v.pop().unwrap_or(0),
            backup_red_key: v.pop().unwrap_or(0),
            backup_green_key: v.pop().unwrap_or(0),
            backup_blue_key: v.pop().unwrap_or(0),
            backup_shield: v.pop().unwrap_or(0),
            backup_treasures: v.pop().unwrap_or(0),
            backup_gems: v.pop().unwrap_or(0),
            backup_portal_stones: v.pop().unwrap_or(0),
            backup_silver_keys: v.pop().unwrap_or(0),
            backup_gold_keys: v.pop().unwrap_or(0),
            backup_t_treasures: v.pop().unwrap_or(0),
            backup_t_gems: v.pop().unwrap_or(0),
            backup_t_portal_stones: v.pop().unwrap_or(0),
            backup_t_silver_keys: v.pop().unwrap_or(0),
            backup_t_gold_keys: v.pop().unwrap_or(0),
            total_gold_doors: v.pop().unwrap_or(0),
            total_silver_doors: v.pop().unwrap_or(0),
            total_teal_doors: v.pop().unwrap_or(0),
            total_purple_doors: v.pop().unwrap_or(0),
            total_red_doors: v.pop().unwrap_or(0),
            total_blue_doors: v.pop().unwrap_or(0),
            total_green_doors: v.pop().unwrap_or(0),
            total_tokens: v.pop().unwrap_or(0),
            total_teal_keys: v.pop().unwrap_or(0),
            total_purple_keys: v.pop().unwrap_or(0),
            witch_phase1: v.pop().unwrap_or(0),
            witch_phase2: v.pop().unwrap_or(0),
            witch_phase3: v.pop().unwrap_or(0),
            bunny_crime_scene: v.pop().unwrap_or(0),
            backup_gem_boots: v.pop().unwrap_or(0),
            overkill: v.pop().unwrap_or(0),
            backdoor_banditry: v.pop().unwrap_or(0),
            dragonslain: v.pop().unwrap_or(0),
            witch_hammer: v.pop().unwrap_or(0),
            witch_water_ring: v.pop().unwrap_or(0),
            witch_lava_charm: v.pop().unwrap_or(0),
            witch_skeleton_key: v.pop().unwrap_or(0),
            witch_compass: v.pop().unwrap_or(0),
            witch_boots: v.pop().unwrap_or(0),
            witch_gem_sword_1: v.pop().unwrap_or(0),
            witch_gem_sword_2: v.pop().unwrap_or(0),
            witch_gem_sword_3: v.pop().unwrap_or(0),
            completion_swamp: v.pop().unwrap_or(0),
            completion_maze: v.pop().unwrap_or(0),
            completion_boots: v.pop().unwrap_or(0),
            completion_cloak: v.pop().unwrap_or(0),
            ngp: v.pop().unwrap_or(0),
            ngp_tokens: v.pop().unwrap_or(0),
            red_sword: v.pop().unwrap_or(0),
            red_shield: v.pop().unwrap_or(0),
            red_gear_skip: v.pop().unwrap_or(0),
            ngpp: v.pop().unwrap_or(0),
            rdragon_killed: v.pop().unwrap_or(0),
            total_ngp_tokens: v.pop().unwrap_or(0),
            prevented_night: v.pop().unwrap_or(0),
            convergence_key: v.pop().unwrap_or(0),
            possum_coins: v.pop().unwrap_or(0),
            total_possum_coins: v.pop().unwrap_or(0),
            backup_t_possum_coins: v.pop().unwrap_or(0),
            backup_possum_coins: v.pop().unwrap_or(0),
            broom: v.pop().unwrap_or(0),
            backup_broom: v.pop().unwrap_or(0),
            collector_eye: v.pop().unwrap_or(0),
            backup_collector_eye: v.pop().unwrap_or(0),
            mirror: v.pop().unwrap_or(0),
            backup_mirror: v.pop().unwrap_or(0),
            dragon_egg: v.pop().unwrap_or(0),
            backup_dragon_egg: v.pop().unwrap_or(0),
            save_crystal: v.pop().unwrap_or(0),
            backup_save_crystal: v.pop().unwrap_or(0),
            carrot: v.pop().unwrap_or(0),
            backup_carrot: v.pop().unwrap_or(0),
            green_sword: v.pop().unwrap_or(0),
            green_shield: v.pop().unwrap_or(0),
            backup_green_sword: v.pop().unwrap_or(0),
            backup_green_shield: v.pop().unwrap_or(0),
            bunny_love: v.pop().unwrap_or(0),
            bunny_level: v.pop().unwrap_or(0),
            hero_form_override: v.pop().unwrap_or(0),
            hero_color_override: v.pop().unwrap_or(0),
            bunny_color_override: v.pop().unwrap_or(0),
            evil_bunny_tamed: v.pop().unwrap_or(0),
            backup_ngp_tokens: v.pop().unwrap_or(0),
            backup_secret_tokens: v.pop().unwrap_or(0),
            backup_t_ngp_tokens: v.pop().unwrap_or(0),
            backup_t_secret_tokens: v.pop().unwrap_or(0),
            fishing_pole: v.pop().unwrap_or(0),
            fish: v.pop().unwrap_or(0),
            ngppp: v.pop().unwrap_or(0),
            rawr_1_map: v.pop().unwrap_or(0),
            rawr_1_x: v.pop().unwrap_or(0),
            rawr_1_y: v.pop().unwrap_or(0),
            rawr_2_map: v.pop().unwrap_or(0),
            rawr_2_x: v.pop().unwrap_or(0),
            rawr_2_y: v.pop().unwrap_or(0),
            rawr_3_map: v.pop().unwrap_or(0),
            rawr_3_x: v.pop().unwrap_or(0),
            rawr_3_y: v.pop().unwrap_or(0),
            rawr: v.pop().unwrap_or(0),
        }
    }
}

impl Inventory {
    pub fn into_vec(&self) -> Vec<u8> {
        vec![
            self.hearts,
            self.swords,
            self.gold_keys,
            self.silver_keys,
            self.red_keys,
            self.blue_keys,
            self.green_keys,
            self.total_swords,
            self.total_gold_keys,
            self.total_silver_keys,
            self.total_red_keys,
            self.total_blue_keys,
            self.total_green_keys,
            self.portal_stones,
            self.gems,
            self.treasures,
            self.total_portal_stones,
            self.total_gems,
            self.total_treasures,
            self.hammers,
            self.boots,
            self.compasses,
            self.castle_puzzle,
            self.castle_entered,
            self.green_key,
            self.bloodmoon_effect,
            self.bloodmoon_count,
            self.bloodmoon_orb_hide,
            self.boss_reached,
            self.castle_labyrinth_open,
            self.swamp_secret,
            self.back_door_lock_1,
            self.back_door_lock_2,
            self.victory_road_solved,
            self.dragon_treasure,
            self.secret_sockets,
            self.secret_tokens,
            self.castle_skip_primed,
            self.castle_puzzle_solved,
            self.warp_portal_01,
            self.warp_portal_02,
            self.warp_portal_03,
            self.warp_portal_04,
            self.warp_portal_05,
            self.warp_portal_06,
            self.warp_portal_07,
            self.warp_portal_08,
            self.warp_portal_09,
            self.warp_portal_10,
            self.ghost_ship_entered,
            self.hermit_sword_acquired,
            self.gem_sword,
            self.gold_sword,
            self.total_steps,
            self.witch_cloak,
            self.lava_charms,
            self.gold_sword_door,
            self.spectacles,
            self.skeleton_key,
            self.water_ring,
            self.shield,
            self.gem_shield,
            self.stopwatch,
            self.snake_boss_defeated,
            self.fairyland_lock_1,
            self.fairyland_lock_2,
            self.fairyland_lock_3,
            self.fairyland_lock_4,
            self.fairyland_lock_5,
            self.fairyland_lock_6,
            self.fairyland_lock_7,
            self.fairyland_lock_8,
            self.fairyland_locks,
            self.gem_heart,
            self.save_count,
            self.dragon_killed,
            self.backup_swords,
            self.backup_gem_sword,
            self.backup_gem_shield,
            self.backup_skeleton_key,
            self.backup_lava_charm,
            self.backup_water_ring,
            self.backup_boots,
            self.backup_spectacles,
            self.backup_gold_sword,
            self.purple_keys,
            self.teal_keys,
            self.witch_health,
            self.backup_hammer,
            self.gem_boots,
            self.backup_compass,
            self.greenfight_lock_1,
            self.greenfight_lock_2,
            self.greenfight_lock_3,
            self.greenfight_lock_4,
            self.greenfight_lock_5,
            self.greenfight_lock_6,
            self.greenfight_lock_7,
            self.greenfight_lock_8,
            self.backup_red_key,
            self.backup_green_key,
            self.backup_blue_key,
            self.backup_shield,
            self.backup_treasures,
            self.backup_gems,
            self.backup_portal_stones,
            self.backup_silver_keys,
            self.backup_gold_keys,
            self.backup_t_treasures,
            self.backup_t_gems,
            self.backup_t_portal_stones,
            self.backup_t_silver_keys,
            self.backup_t_gold_keys,
            self.total_gold_doors,
            self.total_silver_doors,
            self.total_teal_doors,
            self.total_purple_doors,
            self.total_red_doors,
            self.total_blue_doors,
            self.total_green_doors,
            self.total_tokens,
            self.total_teal_keys,
            self.total_purple_keys,
            self.witch_phase1,
            self.witch_phase2,
            self.witch_phase3,
            self.bunny_crime_scene,
            self.backup_gem_boots,
            self.overkill,
            self.backdoor_banditry,
            self.dragonslain,
            self.witch_hammer,
            self.witch_water_ring,
            self.witch_lava_charm,
            self.witch_skeleton_key,
            self.witch_compass,
            self.witch_boots,
            self.witch_gem_sword_1,
            self.witch_gem_sword_2,
            self.witch_gem_sword_3,
            self.completion_swamp,
            self.completion_maze,
            self.completion_boots,
            self.completion_cloak,
            self.ngp,
            self.ngp_tokens,
            self.red_sword,
            self.red_shield,
            self.red_gear_skip,
            self.ngpp,
            self.rdragon_killed,
            self.total_ngp_tokens,
            self.prevented_night,
            self.convergence_key,
            self.possum_coins,
            self.total_possum_coins,
            self.backup_t_possum_coins,
            self.backup_possum_coins,
            self.broom,
            self.backup_broom,
            self.collector_eye,
            self.backup_collector_eye,
            self.mirror,
            self.backup_mirror,
            self.dragon_egg,
            self.backup_dragon_egg,
            self.save_crystal,
            self.backup_save_crystal,
            self.carrot,
            self.backup_carrot,
            self.green_sword,
            self.green_shield,
            self.backup_green_sword,
            self.backup_green_shield,
            self.bunny_love,
            self.bunny_level,
            self.hero_form_override,
            self.hero_color_override,
            self.bunny_color_override,
            self.evil_bunny_tamed,
            self.backup_ngp_tokens,
            self.backup_secret_tokens,
            self.backup_t_ngp_tokens,
            self.backup_t_secret_tokens,
            self.fishing_pole,
            self.fish,
            self.ngppp,
            self.rawr_1_map,
            self.rawr_1_x,
            self.rawr_1_y,
            self.rawr_2_map,
            self.rawr_2_x,
            self.rawr_2_y,
            self.rawr_3_map,
            self.rawr_3_x,
            self.rawr_3_y,
            self.rawr,
        ]
    }

    pub fn into_completion_column(&self) -> Vec<u8> {
        [
            self.hearts,
            self.shield,
            self.total_silver_keys,
            self.silver_keys,
            self.total_silver_doors,
            self.total_gold_keys,
            self.gold_keys,
            self.total_gold_doors,
            self.total_teal_keys,
            self.teal_keys,
            self.total_teal_doors,
            self.total_purple_keys,
            self.purple_keys,
            self.total_purple_doors,
            self.total_portal_stones,
            self.portal_stones,
            self.total_gems,
            self.gems,
            self.total_treasures,
            self.dragon_treasure,
            self.total_tokens,
            self.secret_tokens,
            self.total_ngp_tokens,
            self.ngp_tokens,
            self.total_possum_coins,
            self.compasses,
            self.hammers,
            self.skeleton_key,
            self.gem_sword,
            self.red_sword,
            self.gem_shield,
            self.red_shield,
            self.boots,
            self.gem_boots,
            self.spectacles,
            self.lava_charms,
            self.water_ring,
            self.gem_heart,
            self.total_red_keys,
            self.total_red_doors,
            self.total_blue_keys,
            self.total_blue_doors,
            self.total_green_keys,
            self.green_key,
            self.total_green_doors,
            self.bloodmoon_effect,
            self.bloodmoon_count,
            self.collector_eye,
            self.save_crystal,
            self.carrot,
            self.bunny_love,
            self.bunny_level,
            self.mirror,
            self.broom,
            self.witch_cloak,
            self.dragon_egg,
            self.green_sword,
            self.green_shield,
            self.ngp,
            self.ngpp,
            self.ngppp,
            self.witch_compass,
            self.witch_hammer,
            self.witch_skeleton_key,
            self.witch_gem_sword_1,
            self.witch_gem_sword_2,
            self.witch_gem_sword_3,
            self.witch_boots,
            self.witch_lava_charm,
            self.witch_water_ring,
            self.witch_phase1,
            self.witch_phase2,
            self.witch_phase3,
            self.snake_boss_defeated,
            self.boss_reached,
            self.dragon_killed,
            self.rdragon_killed,
            self.bunny_crime_scene,
            self.fairyland_locks,
            self.greenfight_lock_1 + self.greenfight_lock_2 + self.greenfight_lock_3 + self.greenfight_lock_4 + self.greenfight_lock_5 + self.greenfight_lock_6 + self.greenfight_lock_7 + self.greenfight_lock_8,
            self.back_door_lock_1,
            self.back_door_lock_2,
            self.completion_swamp,
            self.completion_maze,
            self.completion_boots,
            self.completion_cloak,
        ].into()
    }
}
