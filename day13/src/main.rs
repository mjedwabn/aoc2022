mod day13 {
    use std::{io::BufRead, cmp::Ordering};
    use json::{JsonValue, number::Number};

    pub fn sum_of_pair_indices_in_the_right_order(input: &mut dyn BufRead) -> usize {
        let packet_pairs = parse_input_as_packet_pairs(input);
        let size = packet_pairs.len();
        return packet_pairs.iter()
            .map(|(left, right)| is_packet_pair_in_right_order(left, right))
            .zip(1..=size)
            .filter(|(in_right_order, _)| *in_right_order == true)
            .map(|(_, index)| index)
            .sum();
    }

    pub fn decoder_key_for_the_distress_signal(input: &mut dyn BufRead) -> usize {
        let mut packets = parse_input_as_packets(input);
        packets.push(Packet { data: JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Number(Number::from(2))])]), packet_type: PacketType::Divider2 });
        packets.push(Packet { data: JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Number(Number::from(6))])]), packet_type: PacketType::Divider6 });
        
        packets.sort_by(|a, b| compare_packets(&a.data, &b.data));

        let i2 = packets.iter().zip(1..packets.len()).find(|(p, _)| p.packet_type == PacketType::Divider2).map(|(_, i)| i).unwrap();
        let i6 = packets.iter().zip(1..packets.len()).find(|(p, _)| p.packet_type == PacketType::Divider6).map(|(_, i)| i).unwrap();

        return i2 * i6;
    }

    fn parse_input_as_packet_pairs(input: &mut dyn BufRead) -> Vec<(Packet, Packet)> {
        return read_input(input)
            .chunks(3)
            .map(parse_packet_pair)
            .collect();
    }

    fn parse_input_as_packets(input: &mut dyn BufRead) -> Vec<Packet> {
        return read_input(input).iter().filter(|line| *line != "").map(parse_packet).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    fn parse_packet_pair(chunk: &[String]) -> (Packet, Packet) {
        let mut packets = chunk.iter()
            .filter(|line| *line != "")
            .map(parse_packet);

        return (packets.next().unwrap(), packets.next().unwrap());
    }

    fn parse_packet(line: &String) -> Packet {
        return Packet { data: json::parse(line).unwrap(), packet_type: PacketType::Signal };
    }

    fn is_packet_pair_in_right_order(left: &Packet, right: &Packet) -> bool {
        return compare_packets(&left.data, &right.data) == Ordering::Less;
    }

    fn compare_packets(left: &JsonValue, right: &JsonValue) -> Ordering {
        if left.is_number() && right.is_number() {
            return left.as_i32().unwrap().cmp(&right.as_i32().unwrap());
        }
        else if left.is_array() && right.is_array() {
            let mut left_array = left.members();
            let mut right_array = right.members();

            loop {
                let left_element = left_array.next();
                let right_element = right_array.next();

                if left_element.is_none() && right_element.is_none() {
                    break;
                }
                else if left_element.is_none() {
                    return Ordering::Less;
                }
                else if right_element.is_none() {
                    return Ordering::Greater;
                }
                else {
                    let cmp = compare_packets(left_element.unwrap(), right_element.unwrap());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }
        }
        else if left.is_number() && right.is_array() {
            return compare_packets(&JsonValue::Array(vec![left.clone()]), right);
        }
        else if left.is_array() && right.is_number() {
            return compare_packets(left, &JsonValue::Array(vec![right.clone()]));
        }

        return Ordering::Equal;
    }

    struct Packet {
        packet_type: PacketType,
        data: JsonValue
    }

    #[derive(PartialEq)]
    enum PacketType {
        Divider2,
        Divider6,
        Signal
    }
}

#[cfg(test)]
mod tests {
    use crate::day13;
    use std::{fs::File, io::BufReader};

    #[test]
    fn given_two_integers_when_left_is_lower_than_right_then_inputs_are_in_order() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [1]\n\
            [2]".as_bytes()), 1);
    }

    #[test]
    fn given_two_integers_when_left_is_higher_than_right_then_inputs_are_not_in_order() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [2]\n\
            [1]".as_bytes()), 0);
    }

    #[test]
    fn given_two_integers_when_left_is_equal_to_right_then_continue_checking_next_part_of_input() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [31]\n\
            [32]".as_bytes()), 1);
    }

    #[test]
    fn given_two_lists_start_comapre_elements_of_each_list() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [[1]]\n\
            [[2]]".as_bytes()), 1);
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [[2]]\n\
            [[1]]".as_bytes()), 0);
    }

    #[test]
    fn given_two_lists_when_left_list_runs_out_of_items_then_inputs_are_in_order() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [[1]]\n\
            [[1, 1]]".as_bytes()), 1);
    }

    #[test]
    fn given_two_lists_when_right_list_runs_out_of_items_then_inputs_are_not_in_order() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [[1, 1]]\n\
            [[1]]".as_bytes()), 0);
    }

    #[test]
    fn given_exactly_one_value_is_integer_convert_the_integer_to_a_list_containing_that_integer() {
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [[1, 1, 1]]\n\
            [2]".as_bytes()), 1);

            assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut "
            [2]\n\
            [[1, 1, 1]]".as_bytes()), 0);
    }

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut f), 13);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day13::sum_of_pair_indices_in_the_right_order(&mut f), 5506);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day13::decoder_key_for_the_distress_signal(&mut f), 140);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day13::decoder_key_for_the_distress_signal(&mut f), 21756);
    }
}