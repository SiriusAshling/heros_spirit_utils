// pub const OBF: &[u8] = &[11, 232, 4, 13, 199, 22, 40, 7, 114, 175, 20, 99, 170, 248, 221, 5, 109, 18, 227, 127, 239, 200, 1, 34, 120, 151, 234, 129, 198, 88, 12, 196, 77, 100, 148, 82, 240, 208, 186, 39, 173, 155, 168, 43, 126, 241, 47, 37, 201, 185, 134, 252, 230, 218, 167, 6, 237, 165, 68, 209, 174, 176, 41, 188, 70, 56, 187, 142, 107, 207, 136, 49, 8, 211, 105, 93, 228, 210, 183, 54, 104, 28, 144, 106, 193, 214, 242, 59, 131, 3, 29, 133, 57, 55, 141, 250, 115, 45, 158, 128, 75, 97, 194, 24, 27, 85, 184, 181, 205, 78, 153, 190, 146, 121, 235, 95, 46, 2, 87, 246, 81, 233, 182, 66, 180, 73, 215, 202, 212, 63, 23, 53, 223, 226, 65, 122, 19, 79, 159, 123, 172, 245, 83, 102, 220, 243, 36, 51, 86, 191, 96, 52, 0, 58, 61, 118, 169, 90, 31, 147, 222, 163, 224, 42, 103, 219, 69, 16, 251, 76, 195, 206, 64, 177, 138, 60, 225, 111, 32, 157, 92, 10, 236, 143, 238, 50, 91, 110, 25, 154, 119, 140, 145, 124, 150, 48, 213, 161, 67, 164, 156, 204, 84, 247, 80, 14, 130, 98, 62, 94, 152, 160, 149, 253, 15, 192, 33, 179, 178, 30, 229, 203, 249, 137, 166, 255, 72, 217, 26, 101, 71, 113, 244, 89, 74, 21, 135, 117, 44, 17, 125, 108, 132, 189, 162, 231, 197, 112, 116, 38, 139, 216, 254, 171, 9, 35];
pub const DEOBF: [u8; 256] = [152, 22, 117, 89, 2, 15, 55, 7, 72, 254, 181, 0, 30, 3, 205, 214, 167, 239, 17, 136, 10, 235, 5, 130, 103, 188, 228, 104, 81, 90, 219, 158, 178, 216, 23, 255, 146, 47, 249, 39, 6, 62, 163, 43, 238, 97, 116, 46, 195, 71, 185, 147, 151, 131, 79, 93, 65, 92, 153, 87, 175, 154, 208, 129, 172, 134, 123, 198, 58, 166, 64, 230, 226, 125, 234, 100, 169, 32, 109, 137, 204, 120, 35, 142, 202, 105, 148, 118, 29, 233, 157, 186, 180, 75, 209, 115, 150, 101, 207, 11, 33, 229, 143, 164, 80, 74, 83, 68, 241, 16, 187, 177, 247, 231, 8, 96, 248, 237, 155, 190, 24, 113, 135, 139, 193, 240, 44, 19, 99, 27, 206, 88, 242, 91, 50, 236, 70, 223, 174, 250, 191, 94, 67, 183, 82, 192, 112, 159, 34, 212, 194, 25, 210, 110, 189, 41, 200, 179, 98, 138, 211, 197, 244, 161, 199, 57, 224, 54, 42, 156, 12, 253, 140, 40, 60, 9, 61, 173, 218, 217, 124, 107, 122, 78, 106, 49, 38, 66, 63, 243, 111, 149, 215, 84, 102, 170, 31, 246, 28, 4, 21, 48, 127, 221, 201, 108, 171, 69, 37, 59, 77, 73, 128, 196, 85, 126, 251, 227, 53, 165, 144, 14, 160, 132, 162, 176, 133, 18, 76, 220, 52, 245, 1, 121, 26, 114, 182, 56, 184, 20, 36, 45, 86, 145, 232, 141, 119, 203, 13, 222, 95, 168, 51, 213, 252, 225];
// Terrain Flags are: unused, ?, Water Path / Death Tile, Hard Wall, Blocks Movement, Lava, Slow, Blocks Sight
pub const TERRAIN_FLAGS: [u8; 67] = [0b00001001, 0b00001001, 0, 0, 0, 0, 0b00010011, 0, 0, 0, 0, 0, 0, 0, 0b00001001, 0b00001001, 0b00001001, 0b00001001, 0, 0, 0b00000011, 0b00000011, 0b00000010, 0b00000010, 0b00010011, 0b00000011, 0, 0b00000011, 0b00000011, 0, 0b00001010, 0b00000110, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00001010, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b00010011, 0b00001001, 0b00001001];
// pub const PASS: &str = "uqqwptqvOwwuswstp";
pub const BRIGHT_MAPS: [u8; 3] = [8, 15, 16];
pub const SPRITE_TILE_BIT_TABLE: [u8; 114] = [0, 1, 2, 2, 2, 2, 2, 3, 2, 2, 44, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 11, 11, 12, 13, 14, 15, 16, 17, 18, 11, 11, 19, 20, 21, 22, 23, 24, 24, 25, 19, 11, 10, 10, 16, 43, 11, 24, 26, 27, 28, 29, 29, 29, 29, 30, 31, 32, 33, 30, 31, 32, 33, 34, 35, 36, 37, 37, 38, 39, 40, 41, 30, 31, 32, 33, 42, 37, 37, 38, 39, 9, 45, 46, 47, 48, 30, 31, 32, 33, 44, 49, 50, 44, 37, 37, 38, 39, 51, 52, 53, 54, 55, 0, 1, 56, 11, 24, 33, 32, 31, 30];
pub const SPRITE_TILE_BITS: [u16; 228] = [614, 615, 616, 617, 618, 618, 620, 621, 0, 1, 16, 17, 219, 220, 221, 222, 622, 623, 624, 625, 534, 535, 538, 539, 536, 537, 538, 539, 540, 541, 544, 545, 542, 543, 544, 545, 654, 654, 655, 656, 126, 127, 128, 129, 130, 131, 132, 133, 207, 208, 209, 210, 211, 212, 213, 214, 203, 204, 205, 206, 215, 216, 217, 218, 223, 79, 225, 226, 227, 228, 229, 230, 240, 258, 260, 266, 612, 612, 613, 613, 626, 627, 628, 629, 610, 611, 611, 610, 606, 607, 608, 609, 598, 599, 600, 601, 602, 603, 604, 605, 640, 641, 642, 643, 631, 632, 632, 633, 634, 634, 635, 635, 636, 637, 638, 639, 241, 242, 243, 244, 526, 526, 526, 526, 527, 527, 527, 527, 528, 528, 528, 528, 529, 529, 529, 529, 224, 224, 224, 224, 652, 653, 653, 652, 653, 652, 652, 653, 657, 657, 657, 657, 658, 658, 658, 658, 659, 659, 659, 659, 630, 630, 630, 630, 546, 547, 548, 549, 688, 689, 690, 691, 741, 742, 743, 744, 747, 747, 748, 748, 745, 745, 746, 746, 758, 759, 774, 775, 756, 757, 772, 773, 760, 761, 776, 777, 192, 193, 193, 192, 193, 192, 192, 193, 762, 763, 778, 779, 792, 793, 808, 809, 784, 785, 800, 801, 790, 791, 806, 807, 752, 753, 768, 769, 788, 789, 804, 805];
pub const SPRITE_TILE_FLIP_TABLE: [u8; 114] = [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 2, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 5, 5, 5, 5, 5, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 1, 2, 0, 0, 0, 5, 5, 5, 5, 2, 0, 0, 2, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 5, 5, 5, 5];
pub const SPRITE_TILE_FLIPS: [bool; 72] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, true, true, false];
pub const SPRITE_PALETTE_TABLE: [u8; 114] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 59, 6, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 52, 21, 24, 25, 26, 24, 27, 28, 29, 30, 31, 32, 20, 27, 13, 27, 20, 34, 35, 36, 58, 60, 60, 37, 37, 37, 38, 39, 40, 41, 42, 42, 42, 42, 43, 43, 43, 43, 44, 45, 45, 46, 47, 48, 47, 49, 50, 51, 51, 51, 51, 53, 54, 55, 56, 55, 57, 59, 61, 62, 63, 64, 64, 64, 64, 65, 66, 66, 67, 68, 69, 70, 69, 71, 72, 73, 74, 75, 76, 76, 77, 78, 78, 79, 79, 79, 79];
pub const SPRITE_PALETTES: [[u8; 4]; 80] = [
    [25, 57, 43, 64],
    [13, 24, 40, 64],
    [13, 0, 16, 64],
    [13, 6, 22, 64],
    [13, 9, 25, 64],
    [13, 2, 18, 64],
    [13, 8, 24, 64],
    [13, 28, 33, 64],
    [13, 19, 3, 64],
    [8, 42, 55, 64],
    [16, 24, 55, 64],
    [18, 35, 55, 64],
    [12, 23, 55, 64],
    [63, 0, 32, 64],
    [40, 0, 0, 64],
    [32, 0, 0, 64],
    [0, 0, 0, 64],
    [22, 0, 0, 64],
    [25, 0, 0, 64],
    [18, 0, 0, 64],
    [8, 24, 32, 64],
    [8, 24, 0, 64],
    [3, 19, 35, 64],
    [17, 33, 60, 64],
    [8, 24, 40, 64],
    [13, 0, 22, 64],
    [27, 43, 41, 64],
    [17, 33, 59, 64],
    [20, 36, 32, 64],
    [16, 18, 64, 64],
    [22, 40, 32, 64],
    [22, 16, 48, 64],
    [18, 17, 33, 64],
    [63, 0, 32, 64],
    [44, 0, 0, 64],
    [19, 0, 0, 64],
    [12, 28, 60, 64],
    [41, 64, 64, 64],
    [64, 19, 35, 64],
    [3, 19, 35, 64],
    [3, 19, 64, 64],
    [3, 64, 35, 64],
    [3, 19, 35, 50],
    [13, 27, 42, 41],
    [13, 13, 13, 64],
    [16, 49, 32, 64],
    [12, 11, 9, 10],
    [12, 42, 41, 57],
    [12, 39, 56, 55],
    [63, 6, 23, 64],
    [5, 21, 54, 64],
    [1, 17, 33, 60],
    [8, 17, 40, 64],
    [13, 0, 16, 48],
    [12, 7, 21, 5],
    [12, 37, 36, 53],
    [12, 35, 50, 61],
    [63, 37, 53, 64],
    [18, 33, 49, 64],
    [63, 19, 34, 64],
    [7, 6, 22, 64],
    [40, 40, 40, 64],
    [28, 28, 28, 64],
    [21, 21, 21, 64],
    [32, 7, 21, 22],
    [63, 21, 37, 64],
    [13, 0, 45, 16],
    [63, 43, 58, 64],
    [13, 3, 34, 19],
    [13, 33, 49, 59],
    [13, 12, 28, 33],
    [22, 5, 21, 64],
    [8, 24, 39, 64],
    [6, 22, 17, 64],
    [7, 17, 49, 64],
    [17, 33, 60, 64],
    [23, 39, 55, 64],
    [22, 23, 9, 64],
    [11, 26, 42, 64],
    [2, 17, 49, 3],
];
pub const MAP_PALETTE_TABLE: [u8; 67] = [0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 1, 1, 1, 0, 2, 0, 2, 2, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0];
pub const MAP_PALETTES: [u8; 44] = [0, 2, 4, 0, 4, 5, 7, 9, 10, 14, 0, 15, 19, 0, 11, 7, 8, 13, 14, 11, 16, 6, 4, 20, 12, 0, 3, 17, 0, 23, 23, 18, 1, 21, 22, 28, 24, 25, 26, 0, 15, 15, 0, 27];
pub const PALETTES: [[u8; 4]; 145] = [
    [13, 0, 16, 48],
    [13, 8, 24, 25],
    [13, 8, 9, 25],
    [20, 28, 44, 48],
    [20, 22, 40, 23],
    [13, 8, 0, 16],
    [13, 13, 8, 9],
    [13, 13, 11, 9],
    [20, 1, 2, 18],
    [20, 22, 8, 23],
    [13, 8, 55, 55],
    [13, 8, 54, 55],
    [13, 54, 8, 55],
    [20, 49, 33, 32],
    [20, 22, 40, 6],
    [13, 7, 23, 56],
    [13, 23, 56, 56],
    [13, 23, 24, 56],
    [20, 49, 33, 32],
    [20, 22, 40, 6],
    [13, 7, 24, 16],
    [22, 7, 13, 24],
    [7, 8, 22, 24],
    [20, 28, 43, 12],
    [20, 40, 22, 39],
    [13, 7, 38, 54],
    [13, 8, 0, 16],
    [13, 8, 0, 16],
    [20, 28, 49, 12],
    [20, 40, 22, 6],
    [0, 7, 24, 16],
    [22, 7, 13, 24],
    [7, 8, 22, 24],
    [20, 28, 43, 12],
    [20, 40, 22, 39],
    [13, 8, 24, 32],
    [13, 24, 32, 48],
    [13, 24, 9, 48],
    [20, 49, 33, 48],
    [20, 22, 40, 23],
    [13, 8, 24, 32],
    [13, 12, 34, 50],
    [13, 12, 28, 50],
    [20, 49, 33, 50],
    [20, 22, 40, 23],
    [13, 12, 0, 16],
    [13, 19, 35, 43],
    [13, 24, 27, 43],
    [20, 28, 12, 44],
    [20, 22, 40, 23],
    [13, 24, 39, 56],
    [13, 28, 58, 43],
    [13, 28, 53, 43],
    [20, 34, 50, 59],
    [20, 22, 40, 58],
    [13, 0, 16, 49],
    [13, 0, 32, 16],
    [13, 0, 9, 16],
    [20, 33, 59, 49],
    [20, 22, 40, 23],
    [13, 12, 18, 58],
    [13, 4, 36, 3],
    [13, 4, 19, 3],
    [20, 28, 44, 48],
    [20, 22, 40, 23],
    [13, 0, 16, 28],
    [13, 12, 28, 50],
    [13, 12, 37, 50],
    [20, 3, 19, 28],
    [20, 38, 4, 37],
    [13, 0, 16, 48],
    [13, 8, 39, 10],
    [13, 8, 41, 10],
    [20, 9, 17, 26],
    [20, 23, 39, 38],
    [8, 24, 39, 56],
    [13, 8, 24, 10],
    [13, 8, 27, 10],
    [20, 27, 17, 10],
    [20, 22, 40, 23],
    [3, 19, 25, 41],
    [13, 13, 3, 9],
    [13, 13, 19, 13],
    [20, 1, 2, 18],
    [20, 41, 3, 25],
    [13, 45, 54, 58],
    [13, 24, 32, 48],
    [13, 24, 9, 48],
    [20, 28, 11, 43],
    [20, 22, 40, 23],
    [38, 22, 6, 7],
    [39, 23, 6, 7],
    [53, 21, 5, 7],
    [20, 38, 5, 7],
    [20, 22, 40, 23],
    [13, 13, 8, 0],
    [1, 62, 62, 45],
    [1, 13, 16, 13],
    [1, 38, 5, 7],
    [1, 22, 40, 23],
    [8, 13, 24, 57],
    [13, 45, 54, 29],
    [13, 55, 11, 29],
    [20, 12, 17, 2],
    [20, 22, 40, 23],
    [13, 8, 24, 39],
    [13, 8, 38, 56],
    [13, 8, 45, 56],
    [20, 19, 34, 52],
    [20, 22, 40, 58],
    [13, 8, 24, 56],
    [13, 24, 39, 8],
    [13, 24, 9, 8],
    [20, 50, 17, 8],
    [20, 22, 40, 23],
    [13, 0, 16, 48],
    [13, 0, 32, 16],
    [13, 0, 45, 16],
    [0, 0, 16, 32],
    [20, 22, 40, 23],
    [8, 24, 56, 55],
    [13, 9, 57, 11],
    [13, 24, 25, 11],
    [20, 17, 33, 43],
    [20, 22, 40, 23],
    [3, 34, 50, 61],
    [13, 12, 28, 35],
    [13, 7, 52, 35],
    [20, 34, 19, 28],
    [20, 56, 40, 22],
    [32, 16, 13, 5],
    [13, 32, 29, 61],
    [13, 13, 0, 16],
    [20, 34, 19, 28],
    [20, 56, 40, 22],
    [8, 0, 54, 55],
    [13, 8, 0, 42],
    [13, 8, 27, 42],
    [20, 44, 33, 43],
    [20, 22, 40, 23],
    [13, 13, 6, 0],
    [40, 62, 62, 55],
    [0, 16, 38, 13],
    [1, 38, 5, 7],
    [1, 22, 40, 2],
];
pub const COLOR_TABLE: [[u8; 4]; 67] = [
    [124, 124, 124, u8::MAX],
    [0, 0, 252, u8::MAX],
    [0, 0, 188, u8::MAX],
    [68, 40, 188, u8::MAX],
    [148, 0, 132, u8::MAX],
    [168, 0, 32, u8::MAX],
    [168, 16, 0, u8::MAX],
    [136, 20, 0, u8::MAX],
    [80, 48, 0, u8::MAX],
    [0, 120, 0, u8::MAX],
    [0, 104, 0, u8::MAX],
    [0, 88, 0, u8::MAX],
    [0, 64, 88, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [188, 188, 188, u8::MAX],
    [0, 120, 248, u8::MAX],
    [0, 88, 248, u8::MAX],
    [104, 68, 252, u8::MAX],
    [216, 0, 204, u8::MAX],
    [228, 0, 88, u8::MAX],
    [248, 56, 0, u8::MAX],
    [228, 92, 16, u8::MAX],
    [172, 124, 0, u8::MAX],
    [0, 184, 0, u8::MAX],
    [0, 168, 0, u8::MAX],
    [0, 168, 68, u8::MAX],
    [0, 136, 136, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [248, 248, 248, u8::MAX],
    [60, 188, 252, u8::MAX],
    [104, 136, 252, u8::MAX],
    [152, 120, 248, u8::MAX],
    [248, 120, 248, u8::MAX],
    [248, 88, 152, u8::MAX],
    [248, 120, 88, u8::MAX],
    [252, 160, 68, u8::MAX],
    [248, 184, 0, u8::MAX],
    [184, 248, 24, u8::MAX],
    [88, 216, 84, u8::MAX],
    [88, 248, 152, u8::MAX],
    [0, 232, 216, u8::MAX],
    [120, 120, 120, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [252, 252, 252, u8::MAX],
    [164, 228, 252, u8::MAX],
    [184, 184, 248, u8::MAX],
    [216, 184, 248, u8::MAX],
    [248, 184, 248, u8::MAX],
    [248, 164, 192, u8::MAX],
    [240, 208, 176, u8::MAX],
    [252, 224, 168, u8::MAX],
    [248, 216, 120, u8::MAX],
    [216, 248, 120, u8::MAX],
    [184, 248, 184, u8::MAX],
    [184, 248, 216, u8::MAX],
    [0, 252, 252, u8::MAX],
    [248, 216, 248, u8::MAX],
    [24, 24, 24, u8::MAX],
    [24, 24, 24, u8::MAX],
    [0, 0, 0, 0],
    [13, 17, 13, u8::MAX],
    [0, 0, 0, u8::MAX]
];
pub const TILE_16S: [[u16; 4]; 66] = [
    [644, 645, 646, 647],
    [231, 232, 233, 234],
    [235, 236, 237, 238],
    [0, 0, 0, 0],
    [78, 78, 78, 78],
    [2, 3, 18, 19],
    [0, 0, 0, 0],
    [6, 7, 22, 23],
    [8, 9, 24, 25],
    [10, 13, 26, 29],
    [12, 13, 28, 29],
    [14, 15, 30, 31],
    [12, 11, 28, 27],
    [2, 3, 18, 19],
    [4, 5, 20, 21],
    [32, 33, 48, 49],
    [34, 35, 50, 51],
    [36, 37, 52, 53],
    [38, 39, 54, 55],
    [40, 41, 56, 57],
    [41, 40, 57, 56],
    [44, 45, 60, 61],
    [46, 47, 62, 63],
    [32, 33, 48, 49],
    [64, 65, 80, 81],
    [76, 37, 52, 77],
    [0, 0, 0, 0],
    [42, 43, 58, 59],
    [0, 0, 0, 0],
    [66, 67, 82, 83],
    [66, 67, 82, 83],
    [74, 75, 84, 85],
    [68, 71, 84, 87],
    [68, 69, 90, 91],
    [70, 69, 86, 85],
    [74, 73, 84, 87],
    [68, 71, 90, 89],
    [70, 69, 88, 91],
    [72, 75, 86, 85],
    [74, 73, 90, 89],
    [70, 71, 88, 89],
    [72, 75, 88, 91],
    [72, 73, 86, 87],
    [74, 75, 90, 91],
    [70, 71, 86, 87],
    [68, 69, 84, 85],
    [72, 73, 88, 89],
    [74, 75, 84, 85],
    [68, 71, 84, 87],
    [68, 69, 90, 91],
    [70, 69, 86, 85],
    [74, 73, 84, 87],
    [68, 71, 90, 89],
    [70, 69, 88, 91],
    [72, 75, 86, 85],
    [74, 73, 90, 89],
    [70, 71, 88, 89],
    [72, 75, 88, 91],
    [72, 73, 86, 87],
    [74, 75, 90, 91],
    [70, 71, 86, 87],
    [68, 69, 84, 85],
    [72, 73, 88, 89],
    [672, 673, 674, 675],
    [672, 673, 674, 675],
    [676, 677, 678, 67],
];
