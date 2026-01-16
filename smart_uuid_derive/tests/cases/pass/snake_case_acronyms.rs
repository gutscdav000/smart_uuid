//! Test: Snake_case conversion for acronyms
//!
//! Verifies that acronyms are handled correctly:
//! - HTTPServer -> "http_server" (not "h_t_t_p_server")
//! - XMLParser -> "xml_parser" (not "x_m_l_parser")

use smart_uuid::UuidType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, UuidType)]
enum ProtocolType {
    HTTPServer,
    HTTPSClient,
    XMLParser,
    JSONEncoder,
    TCPSocket,
    UDPPacket,
}

fn main() {
    // Verify correct acronym handling
    assert_eq!(ProtocolType::HTTPServer.prefix(), "http_server");
    assert_eq!(ProtocolType::HTTPSClient.prefix(), "https_client");
    assert_eq!(ProtocolType::XMLParser.prefix(), "xml_parser");
    assert_eq!(ProtocolType::JSONEncoder.prefix(), "json_encoder");
    assert_eq!(ProtocolType::TCPSocket.prefix(), "tcp_socket");
    assert_eq!(ProtocolType::UDPPacket.prefix(), "udp_packet");

    println!("Snake_case acronym handling tests passed!");
}
