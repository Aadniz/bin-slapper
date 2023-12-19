use rocket_dyn_templates::Template;
use std::collections::HashMap;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::Request;
use crate::context;


lazy_static! {
    static ref HTTP_STATUS_CODES: HashMap<u16, &'static str> = {
        let mut codes = HashMap::new();

        codes.insert(100, "Continue");
        codes.insert(101, "Switching Protocols");
        codes.insert(102, "Processing");
        codes.insert(200, "OK");
        codes.insert(201, "Created");
        codes.insert(202, "Accepted");
        codes.insert(203, "Non-Authoritative Information");
        codes.insert(204, "No Content");
        codes.insert(205, "Reset Content");
        codes.insert(206, "Partial Content");
        codes.insert(207, "Multi-Status");
        codes.insert(300, "Multiple Choices");
        codes.insert(301, "Moved Permanently");
        codes.insert(302, "Found");
        codes.insert(303, "See Other");
        codes.insert(304, "Not Modified");
        codes.insert(305, "Use Proxy");
        codes.insert(306, "(Unused)");
        codes.insert(307, "Temporary Redirect");
        codes.insert(308, "Permanent Redirect");
        codes.insert(400, "Bad Request");
        codes.insert(401, "Unauthorized");
        codes.insert(402, "Payment Required");
        codes.insert(403, "Forbidden");
        codes.insert(404, "Not Found");
        codes.insert(405, "Method Not Allowed");
        codes.insert(406, "Not Acceptable");
        codes.insert(407, "Proxy Authentication Required");
        codes.insert(408, "Request Timeout");
        codes.insert(409, "Conflict");
        codes.insert(410, "Gone");
        codes.insert(411, "Length Required");
        codes.insert(412, "Precondition Failed");
        codes.insert(413, "Request Entity Too Large");
        codes.insert(414, "Request-URI Too Long");
        codes.insert(415, "Unsupported Media Type");
        codes.insert(416, "Requested Range Not Satisfiable");
        codes.insert(417, "Expectation Failed");
        codes.insert(418, "I'm a teapot");
        codes.insert(419, "Authentication Timeout");
        codes.insert(420, "Enhance Your Calm");
        codes.insert(421, "Misdirected Request");
        codes.insert(422, "Unprocessable Entity");
        codes.insert(423, "Locked");
        codes.insert(424, "Failed Dependency");
        codes.insert(425, "Unordered Collection");
        codes.insert(426, "Upgrade Required");
        codes.insert(428, "Precondition Required");
        codes.insert(429, "Too Many Requests");
        codes.insert(431, "Request Header Fields Too Large");
        codes.insert(449, "Retry With");
        codes.insert(451, "Unavailable For Legal Reasons");
        codes.insert(494, "Request Header Too Large");
        codes.insert(495, "Cert Error");
        codes.insert(496, "No Cert");
        codes.insert(497, "HTTP to HTTPS");
        codes.insert(499, "Client Closed Request");
        codes.insert(500, "Internal Server Error");
        codes.insert(501, "Not Implemented");
        codes.insert(502, "Bad Gateway");
        codes.insert(503, "Service Unavailable");
        codes.insert(504, "Gateway Timeout");
        codes.insert(505, "HTTP Version Not Supported");
        codes.insert(506, "Variant Also Negotiates");
        codes.insert(507, "Insufficient Storage");
        codes.insert(508, "Loop Detected");
        codes.insert(509, "Bandwidth Limit Exceeded");
        codes.insert(510, "Not Extended");
        codes.insert(511, "Network Authentication Required");
        codes.insert(598, "Network read timeout error");
        codes.insert(599, "Network connect timeout error");

        codes
    };
}

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request<'_>) -> Template {

    let msg = match HTTP_STATUS_CODES.get(&status.code) {
        Some(msg) => *msg,
        None => "An unknown error occurred"
    };

    let cxt = context::Sorts::new("Error")
        .status(status.code, msg.to_string());

    Template::render("error", &cxt)
}