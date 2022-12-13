use crate::Packet;

pub fn import_data(data: &str) -> Vec<(Packet, Packet)> {
    data.split_terminator("\n\n")
        .map(|packet_group| {
            let mut both_packets = packet_group.lines().map(|line| parse(line));
            (both_packets.next().unwrap(), both_packets.next().unwrap())
        })
        .collect()
}

pub fn parse(line: &str) -> Packet {
    let mut packet = Packet::List(Box::new(vec![]));

    let mut t: Box<Vec<Packet>> = Box::new(vec![]);

    let mut cursor = &mut t;

    // for character in line.chars() {

    //     let add = Packet::Integer(1);
    //     let mut new_list: Box<Vec<Packet>> = Box::new(vec![]);

    //     cursor.push(Packet::List(new_list.clone()));

    //     cursor = &mut new_list;

    // }





    let add = Packet::Integer(1);

    match &mut packet {
        Packet::Integer(_) => {},
        Packet::List(v) => { v.push(add) }
    };

    packet

}

pub const TEST_DATA: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        // println!("{:?}", input_data);
    }
}
