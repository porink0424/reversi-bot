use crate::{
    calc_legal_places::calc_legal_places,
    constants::{
        WEIGHT_CMOVE, WEIGHT_MOBILITY, WEIGHT_OPENNESS, WEIGHT_STABLE, WEIGHT_WING, WEIGHT_XMOVE,
    },
    get_my_and_opponent_stones::get_my_and_opponent_stones,
    has_game_ended::has_game_ended,
    structs::Board,
};

// Count dangerous CMove
fn calculate_cmove(stones: u64, empty_places: u64) -> i32 {
    let mut count = 0;
    // top left
    if empty_places & 0x8000000000000000 == 0x8000000000000000 {
        count += (stones & 0x4080000000000000).count_ones() as i32
    }
    // top right
    if empty_places & 0x0100000000000000 == 0x0100000000000000 {
        count += (stones & 0x0201000000000000).count_ones() as i32
    }
    // bottom left
    if empty_places & 0x0000000000000080 == 0x0000000000000080 {
        count += (stones & 0x0000000000008040).count_ones() as i32
    }
    // bottom right
    if empty_places & 0x0000000000000001 == 0x0000000000000001 {
        count += (stones & 0x0000000000000102).count_ones() as i32
    }
    count
}

// Count dangerous XMove
fn calculate_xmove(stones: u64, empty_places: u64) -> i32 {
    let mut count = 0;
    // top left
    if empty_places & 0x8000000000000000 == 0x8000000000000000 {
        count += if stones & 0x0040000000000000 != 0 {
            1
        } else {
            0
        }
    }
    // top right
    if empty_places & 0x0100000000000000 == 0x0100000000000000 {
        count += if stones & 0x0002000000000000 != 0 {
            1
        } else {
            0
        }
    }
    // bottom left
    if empty_places & 0x0000000000000080 == 0x0000000000000080 {
        count += if stones & 0x0000000000004000 != 0 {
            1
        } else {
            0
        }
    }
    // bottom right
    if empty_places & 0x0000000000000001 == 0x0000000000000001 {
        count += if stones & 0x0000000000000200 != 0 {
            1
        } else {
            0
        }
    }
    count
}

// Count wing
fn calculate_wing(stones: u64, empty_places: u64) -> i32 {
    let mut count = 0;
    // top
    if ((empty_places & 0x8300000000000000 == 0x8300000000000000)
        && (stones & 0x7c00000000000000 == 0x7c00000000000000))
        || ((empty_places & 0xc100000000000000 == 0xc100000000000000)
            && (stones & 0x3e00000000000000 == 0x3e00000000000000))
    {
        count += 1;
    }
    // bottom
    if ((empty_places & 0x00000000000000c1 == 0x00000000000000c1)
        && (stones & 0x000000000000003e == 0x000000000000003e))
        || ((empty_places & 0x0000000000000083 == 0x0000000000000083)
            && (stones & 0x000000000000007c == 0x000000000000007c))
    {
        count += 1;
    }
    // left
    if ((empty_places & 0x8000000000008080 == 0x8000000000008080)
        && (stones & 0x0080808080800000 == 0x0080808080800000))
        || ((empty_places & 0x8080000000000080 == 0x8080000000000080)
            && (stones & 0x0000808080808000 == 0x0000808080808000))
    {
        count += 1;
    }
    // right
    if ((empty_places & 0x0100000000000101 == 0x0100000000000101)
        && (stones & 0x0001010101010000 == 0x0001010101010000))
        || ((empty_places & 0x0101000000000001 == 0x0101000000000001)
            && (stones & 0x0000010101010100 == 0x0000010101010100))
    {
        count += 1;
    }
    count
}

// Count stable stones
fn calculate_stable(stones: u64) -> i32 {
    let mut count = 0;
    // top left
    if stones & 0x8000000000000000 == 0x8000000000000000 {
        let mask = 0x8000000000000000;
        let mut tmp_count = 1;
        for i in 1..8 {
            // to the bottom
            if stones & (mask >> 8 * i) == (mask >> 8 * i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 8 {
            // if line is filled, divide the tmp_count by 2 to cope for duplication
            tmp_count = 4;
        }
        count += tmp_count;

        let mask = 0x8000000000000000;
        let mut tmp_count = 0;
        for i in 1..8 {
            // to the right
            if stones & (mask >> i) == (mask >> i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 7 {
            tmp_count = 3;
        }
        count += tmp_count;
    }
    // top right
    if stones & 0x0100000000000000 == 0x0100000000000000 {
        let mask = 0x0100000000000000;
        let mut tmp_count = 1;
        for i in 1..8 {
            if stones & (mask >> 8 * i) == (mask >> 8 * i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 8 {
            tmp_count = 4;
        }
        count += tmp_count;

        let mask = 0x0100000000000000;
        let mut tmp_count = 0;
        for i in 1..8 {
            if stones & (mask << i) == (mask << i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 7 {
            tmp_count = 3;
        }
        count += tmp_count;
    }
    // bottom right
    if stones & 0x0000000000000001 == 0x0000000000000001 {
        let mask = 0x0000000000000001;
        let mut tmp_count = 1;
        for i in 1..8 {
            if stones & (mask << 8 * i) == (mask << 8 * i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 8 {
            tmp_count = 4;
        }
        count += tmp_count;

        let mask = 0x0000000000000001;
        let mut tmp_count = 0;
        for i in 1..8 {
            if stones & (mask << i) == (mask << i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 7 {
            tmp_count = 3;
        }
        count += tmp_count;
    }
    // bottom left
    if stones & 0x0000000000000080 == 0x0000000000000080 {
        let mask = 0x0000000000000080;
        let mut tmp_count = 1;
        for i in 1..8 {
            if stones & (mask << 8 * i) == (mask << 8 * i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 8 {
            tmp_count = 4;
        }
        count += tmp_count;

        let mask = 0x0000000000000080;
        let mut tmp_count = 0;
        for i in 1..8 {
            if stones & (mask >> i) == (mask >> i) {
                tmp_count += 1;
                continue;
            }
            break;
        }
        if tmp_count == 7 {
            tmp_count = 3;
        }
        count += tmp_count;
    }
    count
}

// count mobility
fn calculate_mobility(board: &Board) -> i32 {
    let legal_places = calc_legal_places(board);
    return legal_places.count_ones() as i32;
}

// calculate openness of `target_place`
fn calculate_openness(empty_places: u64, target_place: u64) -> i32 {
    let mut count = 0;
    if target_place & 0x00000000000000ff == 0 {
        // there are some places in the bottom
        if (target_place >> 8) & empty_places != 0 {
            // there is an empty place in the bottom
            count += 1;
        }
    }
    if target_place & 0x80808080808080ff == 0 {
        // left bottom
        if (target_place >> 7) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0x8080808080808080 == 0 {
        // left
        if (target_place << 1) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0xff80808080808080 == 0 {
        // left top
        if (target_place << 9) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0xff00000000000000 == 0 {
        // top
        if (target_place << 8) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0xff01010101010101 == 0 {
        // right top
        if (target_place << 7) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0x0101010101010101 == 0 {
        // right
        if (target_place >> 1) & empty_places != 0 {
            count += 1;
        }
    }
    if target_place & 0x01010101010101ff == 0 {
        // right bottom
        if (target_place >> 9) & empty_places != 0 {
            count += 1;
        }
    }
    count
}

pub fn evaluate_normal(board: Board) -> i32 {
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(&board);
    let empty_places = !(my_stones | opponent_stones);

    if has_game_ended(&board) {
        if my_stones.count_ones() > opponent_stones.count_ones() {
            return 100000000;
        } else if my_stones.count_ones() < opponent_stones.count_ones() {
            return -100000000;
        } else {
            return 0;
        }
    }

    let my_cmove = calculate_cmove(my_stones, empty_places);
    let opponent_cmove = calculate_cmove(opponent_stones, empty_places);

    let my_xmove = calculate_xmove(my_stones, empty_places);
    let opponent_xmove = calculate_xmove(opponent_stones, empty_places);

    let my_wing = calculate_wing(my_stones, empty_places);
    let opponent_wing = calculate_wing(opponent_stones, empty_places);

    let my_stable = calculate_stable(my_stones);
    let opponent_stable = calculate_stable(opponent_stones);

    let my_mobility = calculate_mobility(&board);
    let opponent_mobility = calculate_mobility(&board);

    let mut my_openness = 0;
    let mut opponent_openness = 0;
    let mut mask: u64 = 0x8000000000000000;
    for _ in 0..64 {
        if my_stones & mask != 0 {
            my_openness += calculate_openness(empty_places, mask);
        } else if opponent_stones & mask != 0 {
            opponent_openness += calculate_openness(empty_places, mask);
        }
        mask = mask >> 1;
    }

    return (my_cmove - opponent_cmove) * WEIGHT_CMOVE
        + (my_xmove - opponent_xmove) * WEIGHT_XMOVE
        + (my_wing - opponent_wing) * WEIGHT_WING
        + (my_stable - opponent_stable) * WEIGHT_STABLE
        + (my_mobility - opponent_mobility) * WEIGHT_MOBILITY
        + (my_openness - opponent_openness) * WEIGHT_OPENNESS;
}
