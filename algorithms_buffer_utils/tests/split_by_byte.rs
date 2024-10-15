use algorithms_buffer_utils::FastBufferUtils;

#[test]
fn split_short() {
  for (line1, line2) in SHORT_SEMICOLONS
    .fast_split_by_byte(b';')
    .zip(SHORT_SEMICOLONS.split(|e| e == ';'))
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

#[test]
fn no_split_0_to_7() {
  for (line1, line2) in NO_SPLIT_0_TO_7
    .fast_split_by_byte(b'\n')
    .zip(NO_SPLIT_0_TO_7.lines())
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

#[test]
fn no_split_8_to_63() {
  for (line1, line2) in NO_SPLIT_8_TO_63
    .fast_split_by_byte(b'\n')
    .zip(NO_SPLIT_8_TO_63.lines())
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

#[test]
fn no_split() {
  for (line1, line2) in NO_SPLIT.fast_split_by_byte(b'\n').zip(NO_SPLIT.lines()) {
    assert_eq!(line1, line2.as_bytes())
  }
}

#[test]
fn split_lines() {
  for (line1, line2) in MULTILINE_TEXT
    .fast_split_by_byte(b'\n')
    .zip(MULTILINE_TEXT.lines())
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

// Hyper Text Coffee Pot Control Protocol test, because why not.
#[test]
fn split_lines_rfc2324() {
  for (line1, line2) in HTCPCP_RFC_2324
    .fast_split_by_byte(b'\n')
    .zip(HTCPCP_RFC_2324.lines())
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

#[test]
fn split_by_semicolon() {
  for (line1, line2) in SEMICOLON_SEPARATED_TEXT
    .fast_split_by_byte(b';')
    .zip(SEMICOLON_SEPARATED_TEXT.split(';'))
  {
    assert_eq!(line1, line2.as_bytes())
  }
}

const SHORT_SEMICOLONS: &'static str = "a;b;c;d";
const EMPTY: &'static str = "a;b;c;d";
const NO_SPLIT_0_TO_7: &'static str = "aaaaaa";
const NO_SPLIT_8_TO_63: &'static str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const NO_SPLIT: &'static str =
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MULTILINE_TEXT: &'static str = "Sentence 0
Sentence 1
Sentence 2
Sentence 3
Sentence 4
Sentence 5
Sentence 7
Sentence 8
Sentence 9
Sentence 10


Sentence 11 after empty block
Sentence 12 after empty block
Sentence 13 after empty block

";
const SEMICOLON_SEPARATED_TEXT: &'static str = "Value1;Value 2;Config=Test;DoesItWork?=Yes;Hello=World;Crab;Remilia sends her regards;Lorem? vs ipsum;How to make texts longer;    let's try some spaces   ; Down Down Left Up Right;;Yes Double semicolons";
const HTCPCP_RFC_2324: &'static str =
  "Network Working Group                                       L. Masinter
  Request for Comments: 2324                                 1 April 1998
Category: Informational


          Hyper Text Coffee Pot Control Protocol (HTCPCP/1.0)

Status of this Memo

   This memo provides information for the Internet community.  It does
   not specify an Internet standard of any kind.  Distribution of this
   memo is unlimited.

Copyright Notice

   Copyright (C) The Internet Society (1998).  All Rights Reserved.

Abstract

   This document describes HTCPCP, a protocol for controlling,
   monitoring, and diagnosing coffee pots.

1. Rationale and Scope

   There is coffee all over the world. Increasingly, in a world in which
   computing is ubiquitous, the computists want to make coffee. Coffee
   brewing is an art, but the distributed intelligence of the web-
   connected world transcends art.  Thus, there is a strong, dark, rich
   requirement for a protocol designed espressoly for the brewing of
   coffee. Coffee is brewed using coffee pots.  Networked coffee pots
   require a control protocol if they are to be controlled.

   Increasingly, home and consumer devices are being connected to the
   Internet. Early networking experiments demonstrated vending devices
   connected to the Internet for status monitoring [COKE]. One of the
   first remotely _operated_ machine to be hooked up to the Internet,
   the Internet Toaster, (controlled via SNMP) was debuted in 1990
   [RFC2235].

   The demand for ubiquitous appliance connectivity that is causing the
   consumption of the IPv4 address space. Consumers want remote control
   of devices such as coffee pots so that they may wake up to freshly
   brewed coffee, or cause coffee to be prepared at a precise time after
   the completion of dinner preparations.







Masinter                     Informational                      [Page 1]

RFC 2324                       HTCPCP/1.0                   1 April 1998


   This document specifies a Hyper Text Coffee Pot Control Protocol
   (HTCPCP), which permits the full request and responses necessary to
   control all devices capable of making the popular caffeinated hot
   beverages.

   HTTP 1.1 ([RFC2068]) permits the transfer of web objects from origin
   servers to clients. The web is world-wide.  HTCPCP is based on HTTP.
   This is because HTTP is everywhere. It could not be so pervasive
   without being good. Therefore, HTTP is good. If you want good coffee,
   HTCPCP needs to be good. To make HTCPCP good, it is good to base
   HTCPCP on HTTP.

   Future versions of this protocol may include extensions for espresso
   machines and similar devices.

2. HTCPCP Protocol

   The HTCPCP protocol is built on top of HTTP, with the addition of a
   few new methods, header fields and return codes.  All HTCPCP servers
   should be referred to with the \"coffee:\" URI scheme (Section 4).

2.1 HTCPCP Added Methods

2.1.1 The BREW method, and the use of POST

   Commands to control a coffee pot are sent from client to coffee
   server using either the BREW or POST method, and a message body with
   Content-Type set to \"application/coffee-pot-command\".

   A coffee pot server MUST accept both the BREW and POST method
   equivalently.  However, the use of POST for causing actions to happen
   is deprecated.

   Coffee pots heat water using electronic mechanisms, so there is no
   fire. Thus, no firewalls are necessary, and firewall control policy
   is irrelevant. However, POST may be a trademark for coffee, and so
   the BREW method has been added. The BREW method may be used with
   other HTTP-based protocols (e.g., the Hyper Text Brewery Control
   Protocol).

2.1.2 GET method

   In HTTP, the GET method is used to mean \"retrieve whatever
   information (in the form of an entity) identified by the Request-
   URI.\" If the Request-URI refers to a data-producing process, it is
   the produced data which shall be returned as the entity in the
   response and not the source text of the process, unless that text
   happens to be the output of the process.



Masinter                     Informational                      [Page 2]

RFC 2324                       HTCPCP/1.0                   1 April 1998


   In HTCPCP, the resources associated with a coffee pot are physical,
   and not information resources. The \"data\" for most coffee URIs
   contain no caffeine.

2.1.3 PROPFIND method

   If a cup of coffee is data, metadata about the brewed resource is
   discovered using the PROPFIND method [WEBDAV].

2.1.4 WHEN method

   When coffee is poured, and milk is offered, it is necessary for the
   holder of the recipient of milk to say \"when\" at the time when
   sufficient milk has been introduced into the coffee. For this
   purpose, the \"WHEN\" method has been added to HTCPCP. Enough? Say
   WHEN.

2.2 Coffee Pot Header fields

   HTCPCP recommends several HTTP header fields and defines some new
   ones.

2.2.1 Recommended header fields

2.2.1.1 The \"safe\" response header field.

   [SAFE] defines a HTTP response header field, \"Safe\", which can be
   used to indicate that repeating a HTTP request is safe. The inclusion
   of a \"Safe: Yes\" header field allows a client to repeat a previous
   request if the result of the request might be repeated.

   The actual safety of devices for brewing coffee varies widely, and
   may depend, in fact, on conditions in the client rather than just in
   the server. Thus, this protocol includes an extension to the \"Safe\"
   response header:

          Safe                = \"Safe\" \":\" safe-nature
          safe-nature         = \"yes\" | \"no\" | conditionally-safe
          conditionally-safe  = \"if-\" safe-condition
          safe-condition      = \"user-awake\" | token

   indication will allow user agents to handle retries of some safe
   requests, in particular safe POST requests, in a more user-friendly
   way.
";
