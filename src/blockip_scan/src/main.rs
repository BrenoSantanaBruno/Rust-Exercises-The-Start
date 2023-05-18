let network = ipnetwork::Ipv4Network::new([192, 168, 0, 0].into(), 24).unwrap();
use pnet::util::checksum;
use pnet::packet::{Packet, MutablePacket, icmp};

let mut payload = [0u8; 4]; // Define um payload vazio
let packet = icmp::MutableEchoRequestPacket::new(&mut buffer[..]).unwrap();
packet.set_sequence_number(1); // Define o número do pacote
packet.set_identifier(1); // Define o identificador
packet.set_payload(&payload); // Seta o payload

// Envia o pacote para cada endereço dentro do bloco
for ip in network.iter() {
let mut buffer = [0u8; 64];
let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer[..]).unwrap();
ethernet_packet.set_destination(MacAddr::broadcast()); // Endereço MAC de broadcast
ethernet_packet.set_source(my_mac_address); // Seu endereço MAC
ethernet_packet.set_ethertype(EtherTypes::Ipv4);

let mut ipv4_packet = MutableIpv4Packet::new(&mut buffer[ethernet_packet.packet_size()..]).unwrap();
ipv4_packet.set_version(4); // Define a versão do protocolo IP
ipv4_packet.set_header_length(5); // Define o tamanho do header em palavras de 32 bits
ipv4_packet.set_total_length(28); // Define o tamanho total em bytes (28 = 20(header) + 8(ICMP))
ipv4_packet.set_ttl(64); // Define o TTL
ipv4_packet.set_protocol(IpNextHeaderProtocols::Icmp);
ipv4_packet.set_source(my_ip_address); // Seu endereço IP
ipv4_packet.set_destination(ip); // Define o endereço de destino

// Calcula o checksum
let checksum = checksum(&ipv4_packet.to_immutable()[..]);
ipv4_packet.set_checksum(checksum);

packet.populate(&mut icmp_buffer[..]); // Preenche o pacote ICMP com os dados definidos acima

send_packet(&mut socket, &ethernet_packet); // Envia o pacote
}
