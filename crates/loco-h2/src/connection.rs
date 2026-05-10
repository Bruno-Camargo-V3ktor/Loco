/// Prefácio de conexão obrigatório enviado pelo cliente.
/// RFC 9113, Seção 3.4.
pub const PREFACE: &[u8; 24] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
