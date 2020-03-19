//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

#![allow(non_snake_case)]
extern crate zkgroup;

use curve25519_dalek::ristretto::RistrettoPoint;
use sha2::Sha256;

#[test]
fn test_lizard() {
    let p = RistrettoPoint::lizard_encode::<Sha256>(&zkgroup::common::constants::TEST_ARRAY_16);
    let data_out = p.lizard_decode::<Sha256>();
    assert!(data_out.unwrap() == zkgroup::common::constants::TEST_ARRAY_16);
}
pub const AUTH_CREDENTIAL_PRESENTATION_RESULT: [u8; zkgroup::AUTH_CREDENTIAL_PRESENTATION_LEN] = [
    0xc0, 0x51, 0xa9, 0xc1, 0x4d, 0xd9, 0x80, 0xbb, 0xc6, 0x6f, 0x30, 0x98, 0x0f, 0xb9, 0x95, 0x74,
    0x09, 0x38, 0x30, 0xba, 0x52, 0x65, 0xf7, 0x87, 0x1f, 0x57, 0x23, 0xa6, 0x87, 0xdc, 0x75, 0x35,
    0xca, 0x44, 0x2d, 0x45, 0xf9, 0xf4, 0x39, 0xc9, 0x32, 0x76, 0x46, 0x13, 0x95, 0x52, 0x75, 0xd8,
    0x22, 0xbd, 0x3a, 0xb1, 0xf2, 0x91, 0x04, 0xc0, 0x35, 0x8d, 0xa2, 0x9f, 0x83, 0x15, 0x05, 0x2d,
    0x7c, 0x5f, 0x9f, 0x31, 0x95, 0xe5, 0xed, 0x27, 0xfc, 0xaf, 0x25, 0x56, 0x4e, 0xb3, 0xc4, 0x19,
    0xda, 0x5b, 0x0d, 0x7e, 0x2f, 0x4f, 0x4b, 0xaa, 0x70, 0xc1, 0x2d, 0xc1, 0x21, 0x74, 0xe3, 0x7d,
    0x50, 0xc2, 0xa9, 0x3b, 0x80, 0x3d, 0x8c, 0x97, 0x2f, 0x12, 0xaf, 0x02, 0x6a, 0xb6, 0x0d, 0x91,
    0xa4, 0xc4, 0xba, 0x54, 0x84, 0x2e, 0x73, 0x07, 0x3c, 0x0b, 0x52, 0x0f, 0x25, 0x13, 0x10, 0x5a,
    0x6e, 0x72, 0x35, 0x02, 0x04, 0xc1, 0x8d, 0x68, 0x66, 0x45, 0xca, 0x49, 0x91, 0xac, 0x56, 0xa5,
    0x57, 0xf1, 0xe0, 0x5f, 0x77, 0x16, 0x97, 0xfb, 0x6a, 0xef, 0x6d, 0x1d, 0xb7, 0x2e, 0x63, 0x25,
    0xe0, 0xd3, 0x97, 0xfb, 0x79, 0x51, 0xfd, 0xfa, 0x1e, 0x3a, 0xe3, 0x28, 0x24, 0x42, 0x63, 0xd4,
    0x8b, 0x4a, 0x74, 0xde, 0x88, 0xe5, 0x4f, 0xc8, 0xb8, 0x42, 0x7b, 0xf5, 0x69, 0x11, 0xf0, 0x4b,
    0xf2, 0x8f, 0x0a, 0xc8, 0x7f, 0x54, 0x9b, 0x33, 0xc3, 0x24, 0xb0, 0x06, 0x39, 0x13, 0x19, 0x16,
    0x86, 0xb7, 0xbb, 0xc8, 0x72, 0x46, 0x36, 0xd9, 0x1b, 0x56, 0x7c, 0xb7, 0x54, 0x3c, 0x66, 0x39,
    0x48, 0x56, 0xa7, 0x0e, 0xbc, 0x0f, 0x09, 0xdb, 0x48, 0x92, 0x49, 0x9c, 0xe6, 0x25, 0xbd, 0xd3,
    0xfe, 0x42, 0xe3, 0xd8, 0xd8, 0x01, 0x2c, 0xc5, 0x0f, 0xcf, 0x76, 0xf5, 0x8a, 0xb5, 0xfc, 0x39,
    0x20, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x05, 0xbf, 0xfd, 0x51, 0xb8, 0x46, 0x43,
    0x39, 0xda, 0x56, 0x2b, 0xb0, 0x0e, 0xfb, 0xe2, 0xfd, 0xf2, 0xa3, 0x9f, 0xaa, 0xc8, 0x90, 0xb7,
    0xcc, 0xf2, 0xa8, 0xf7, 0x8e, 0xe3, 0x92, 0x03, 0x47, 0x57, 0x4f, 0x11, 0xf0, 0xce, 0x50, 0x37,
    0x0e, 0x85, 0x46, 0x60, 0x94, 0x3f, 0x28, 0x47, 0x4e, 0xae, 0x61, 0xc0, 0x89, 0xb2, 0x4d, 0xed,
    0xa5, 0xd2, 0x72, 0xb6, 0x61, 0x5a, 0x75, 0x00, 0x75, 0x3a, 0x3b, 0x54, 0xc7, 0xd1, 0x47, 0x00,
    0x5b, 0x6b, 0xf8, 0x94, 0x6e, 0x2a, 0x24, 0x26, 0xe6, 0xff, 0x22, 0xd2, 0x7f, 0x5e, 0x8d, 0x01,
    0x1b, 0x5a, 0x22, 0x6d, 0xd8, 0xa4, 0x3c, 0x0b, 0x94, 0xf9, 0x13, 0x14, 0xfe, 0x0b, 0x29, 0xa1,
    0xc8, 0x85, 0xd9, 0x20, 0x25, 0x1b, 0xb4, 0xfb, 0x72, 0xce, 0xb0, 0x2a, 0x31, 0x98, 0xc2, 0x0f,
    0xdb, 0x7c, 0xe3, 0x87, 0xdf, 0x2f, 0x27, 0x0a, 0xc7, 0x7f, 0x81, 0x61, 0x11, 0xda, 0x46, 0xa6,
    0xdf, 0x6d, 0xb0, 0x5e, 0xaa, 0xed, 0x02, 0x6e, 0x73, 0xf8, 0x0d, 0x5b, 0xf0, 0x93, 0x60, 0xb8,
    0x57, 0x49, 0x7f, 0xe2, 0xc2, 0x59, 0x54, 0x05, 0x8b, 0x54, 0x6c, 0xa9, 0x76, 0x6f, 0x39, 0xc5,
    0xda, 0xfe, 0x92, 0x70, 0x9a, 0x11, 0x50, 0x97, 0x27, 0x1b, 0x8d, 0xc1, 0x66, 0x03, 0x3b, 0x0d,
    0x1d, 0xc4, 0x35, 0xb6, 0x2a, 0xfc, 0x50, 0x00, 0x7b, 0x6c, 0xc6, 0xc1, 0xdb, 0xe2, 0x10, 0x13,
    0x16, 0x4e, 0x36, 0x2c, 0x14, 0xf5, 0x09, 0x21, 0xca, 0x2c, 0x5b, 0x59, 0xa1, 0xfd, 0xf1, 0xea,
    0x72, 0x21, 0x44, 0xa9, 0xb5, 0x7f, 0xfc, 0x09, 0x80, 0xad, 0xf5, 0x8c, 0x47, 0x1a, 0xfb, 0xb2,
    0xf9, 0xce, 0xee, 0x8b, 0x17, 0x11, 0x0a, 0x39, 0x22, 0x40, 0x01, 0xdb, 0x90, 0x9e, 0x1a, 0xe5,
    0x2b, 0x89, 0xf5, 0xb6, 0x91, 0x1d, 0x39, 0x02, 0xfe, 0xe7, 0x7c, 0x0c, 0x13, 0xa6, 0x0f, 0xb2,
    0x26, 0x5e, 0x3f, 0x09, 0x55, 0xa6, 0xce, 0xb1, 0x43, 0xf5, 0x31, 0xa9, 0xbc, 0x75, 0x22, 0x8d,
    0xb4, 0x20, 0xbb, 0xd9, 0x53, 0x5f, 0x99, 0x0a, 0x20, 0x47, 0xff, 0xdb, 0x43, 0xa7, 0xb8, 0x02,
    0x03, 0x0e, 0x20, 0xc5, 0x00, 0x51, 0xed, 0xcf, 0x0f, 0xb9, 0x21, 0x78, 0xe0, 0x87, 0xfa, 0x5e,
    0x5d, 0x1e, 0x04, 0x5b, 0x38, 0xa0, 0x1b, 0x3a, 0xf4, 0x4c, 0x19, 0x2e, 0x9e, 0xa2, 0x95, 0x81,
    0xe9, 0xc3, 0xbe, 0xfd, 0x1a, 0x10, 0xaa, 0xb2, 0x0c, 0xcd, 0x19, 0xa6, 0xde, 0xc3, 0xca, 0x06,
    0x68, 0xd8, 0x87, 0xe8, 0xf1, 0x86, 0x85, 0x1e, 0x40, 0xe2, 0x01, 0x00,
];

pub const PROFILE_KEY_CREDENTIAL_PRESENTATION_RESULT: [u8;
    zkgroup::PROFILE_KEY_CREDENTIAL_PRESENTATION_LEN] = [
    0x30, 0xc7, 0xb1, 0xe8, 0x50, 0x9d, 0xe1, 0x65, 0xc1, 0x7e, 0x48, 0x58, 0x32, 0xa2, 0xbb, 0xde,
    0xe5, 0x2c, 0x56, 0x46, 0xe4, 0x38, 0x0a, 0x9e, 0x09, 0x90, 0x93, 0x43, 0xe4, 0xab, 0xcf, 0x52,
    0x70, 0xad, 0x9b, 0xa7, 0x19, 0x49, 0x65, 0xfd, 0x62, 0x9f, 0x47, 0x3c, 0x8b, 0x56, 0xbf, 0x28,
    0xd2, 0xcf, 0xc4, 0x6c, 0x8a, 0x32, 0xc4, 0xd3, 0x91, 0xb8, 0xb1, 0x95, 0xfe, 0x8b, 0x64, 0x41,
    0xa6, 0x04, 0x35, 0x23, 0x7e, 0x98, 0x7f, 0x46, 0xd0, 0x17, 0x83, 0x25, 0x28, 0xd7, 0x0b, 0x5d,
    0x67, 0x5a, 0x49, 0xed, 0x17, 0x00, 0x29, 0xb7, 0xd7, 0x8a, 0xdd, 0xff, 0xe7, 0xd8, 0x73, 0x3f,
    0x90, 0xef, 0xce, 0x84, 0x0c, 0x49, 0x8f, 0x30, 0xe3, 0xd6, 0x3d, 0xb3, 0x8c, 0xd3, 0xe9, 0xa6,
    0x3e, 0x7c, 0xfd, 0xfc, 0x92, 0x23, 0xff, 0x9a, 0xbd, 0x46, 0xab, 0x49, 0x09, 0x17, 0xf9, 0x35,
    0x86, 0xc5, 0xf3, 0x34, 0x66, 0x6b, 0x37, 0xe6, 0xa6, 0xcb, 0x97, 0x0b, 0x12, 0xf8, 0xff, 0x03,
    0x13, 0x50, 0xdb, 0xf5, 0x33, 0x71, 0x09, 0x95, 0x20, 0xe7, 0xa9, 0x71, 0xa0, 0x84, 0x3e, 0x11,
    0x0a, 0xba, 0xbf, 0xf7, 0x9b, 0x27, 0x97, 0x27, 0xb0, 0xb1, 0x02, 0x4d, 0x85, 0x8d, 0xe2, 0x55,
    0x05, 0x01, 0x12, 0x13, 0x01, 0x31, 0x52, 0xcb, 0x2f, 0xc8, 0xfa, 0xce, 0xd9, 0x4e, 0xed, 0x16,
    0xec, 0xed, 0x97, 0xd0, 0xf9, 0x3a, 0xd1, 0xcf, 0xf2, 0x66, 0xb1, 0xb1, 0x57, 0x0a, 0x83, 0x5d,
    0x31, 0xe6, 0x7f, 0xda, 0xa6, 0x76, 0x9b, 0xa2, 0xfe, 0x62, 0x66, 0xf3, 0xab, 0x21, 0xd3, 0x3b,
    0x34, 0xe9, 0x90, 0x43, 0x44, 0x6a, 0x25, 0xa0, 0xb7, 0xc5, 0x5d, 0x5f, 0xb6, 0x34, 0x4b, 0x20,
    0xdf, 0xde, 0x4f, 0x29, 0x57, 0x27, 0x6b, 0xd7, 0x3b, 0x0b, 0x5c, 0x0e, 0xe0, 0xd4, 0xa4, 0x03,
    0xee, 0x9d, 0x2b, 0xfe, 0x76, 0x15, 0x01, 0x7f, 0xd3, 0xa3, 0x19, 0xbd, 0x8b, 0x93, 0xdd, 0x92,
    0x49, 0xc1, 0xb0, 0xde, 0xab, 0x60, 0x8d, 0x1b, 0x8b, 0x03, 0x58, 0x4f, 0x1e, 0x1f, 0x49, 0x3e,
    0x5a, 0x17, 0xa3, 0x2b, 0xe0, 0x84, 0x43, 0xc8, 0x91, 0x5d, 0x6a, 0xf3, 0xa2, 0xcc, 0xc6, 0x1e,
    0x95, 0x96, 0x3d, 0xb5, 0x9a, 0x6d, 0xc0, 0x85, 0xd7, 0xde, 0x37, 0x1f, 0x1f, 0xd9, 0xb9, 0x61,
    0x0a, 0x3d, 0x90, 0x94, 0x0d, 0x24, 0x59, 0x6f, 0xc6, 0xa8, 0x85, 0xd2, 0x4c, 0x4f, 0xba, 0xab,
    0xa7, 0xb8, 0xa4, 0x55, 0x34, 0x9b, 0x5c, 0x3d, 0xb6, 0x49, 0xf5, 0xc8, 0x34, 0x6a, 0x52, 0x6c,
    0xc0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0x76, 0x82, 0xd5, 0x75, 0xa1, 0x31, 0x6e,
    0x0d, 0xcf, 0x93, 0x1a, 0x2f, 0x5b, 0x7d, 0x33, 0xe9, 0x57, 0xd7, 0xc3, 0xdd, 0x21, 0xf0, 0xae,
    0x6a, 0x7f, 0xf4, 0x49, 0xec, 0x74, 0xcd, 0x04, 0xb8, 0xac, 0xdd, 0xbf, 0x65, 0x53, 0x4a, 0x93,
    0xa6, 0x83, 0xc2, 0x68, 0xfb, 0xed, 0xec, 0x33, 0x91, 0xa8, 0x9b, 0x9e, 0x54, 0x88, 0x3d, 0xc5,
    0x00, 0x29, 0x5f, 0xf0, 0x7a, 0x73, 0x8d, 0x0c, 0xf2, 0xac, 0xc1, 0xf4, 0x7d, 0x29, 0x45, 0x05,
    0x31, 0xca, 0xd4, 0x50, 0x4f, 0xc6, 0x40, 0xc0, 0x68, 0x60, 0x15, 0xf2, 0xae, 0x69, 0xf9, 0x1e,
    0x70, 0xbb, 0x41, 0x61, 0x0e, 0x9b, 0x97, 0x05, 0xc9, 0xb6, 0x91, 0xa1, 0x86, 0x8e, 0xd7, 0x49,
    0xda, 0xb9, 0xb1, 0xaf, 0x00, 0x9b, 0x31, 0xc0, 0x5b, 0xa1, 0x21, 0x33, 0x38, 0x46, 0x47, 0x91,
    0x26, 0x22, 0x58, 0x3e, 0x77, 0x20, 0x15, 0x0d, 0x9c, 0xc7, 0xbe, 0xdc, 0xd6, 0xa5, 0x97, 0xb4,
    0x05, 0x36, 0x3c, 0xb5, 0xb9, 0xbd, 0xba, 0x67, 0x07, 0xf8, 0xf6, 0x1f, 0xcd, 0x7a, 0xa7, 0x80,
    0x22, 0x32, 0xc0, 0x07, 0xd8, 0x9b, 0x31, 0x02, 0xe8, 0xfb, 0x5b, 0x7f, 0xbd, 0xce, 0x05, 0xd5,
    0xe3, 0x39, 0xdf, 0xee, 0xe2, 0x38, 0x67, 0x38, 0x2e, 0xba, 0xb6, 0xfb, 0x4a, 0x1d, 0x18, 0x5e,
    0xaf, 0x66, 0x36, 0x80, 0x1f, 0x1b, 0x44, 0x0b, 0x43, 0xdd, 0xdb, 0x9a, 0xf6, 0x2d, 0xcd, 0x39,
    0xa2, 0xb2, 0x0b, 0x42, 0x51, 0x4e, 0x28, 0x5a, 0x96, 0xbf, 0x48, 0x48, 0x69, 0x7a, 0xec, 0x1c,
    0x1f, 0x74, 0x7d, 0x17, 0x91, 0x56, 0x02, 0x0d, 0x16, 0x05, 0xb2, 0x53, 0xbe, 0x3c, 0x71, 0x5f,
    0x0d, 0x52, 0x49, 0x4d, 0xe4, 0xb0, 0xc8, 0x28, 0x16, 0x7a, 0x58, 0xd8, 0x09, 0xdb, 0xec, 0xf1,
    0x7d, 0x16, 0x7b, 0x19, 0xfd, 0x36, 0x17, 0x07, 0xc8, 0xba, 0x1c, 0x33, 0x14, 0x8e, 0xe7, 0xbc,
    0xd4, 0x3a, 0xad, 0x9d, 0xd1, 0x06, 0x03, 0x1b, 0x29, 0xf0, 0x4c, 0x15, 0x2a, 0x6c, 0xce, 0x25,
    0x5d, 0x19, 0x11, 0xa7, 0x6f, 0x86, 0xd6, 0x0e, 0xc2, 0x10, 0x0d, 0xad, 0xe1, 0x6e, 0x46, 0x3c,
    0x53, 0xc2, 0x44, 0xe5, 0xe3, 0x9a, 0x50, 0xdd, 0x55, 0x75, 0x10, 0xf7, 0xbe, 0xc2, 0x0b, 0x70,
    0x0c, 0xda, 0xe4, 0x7f, 0xc7, 0xca, 0xbe, 0x0d, 0xa5, 0xd5, 0x35, 0x98, 0xcd, 0xd5, 0x14, 0xef,
    0x70, 0xe2, 0xc0, 0x6e, 0xef, 0xdc, 0xf4, 0xf6, 0x65, 0xb7, 0x53, 0xc2, 0xae, 0x45, 0x59, 0x18,
    0x13, 0x5c, 0x2b, 0x66, 0x09, 0xbe, 0x62, 0x08, 0x0d, 0xc4, 0x4b, 0x1b, 0xe5, 0xd5, 0xfd, 0x4f,
    0x23, 0xe9, 0x35, 0x21, 0x7b, 0xc1, 0xbe, 0x96, 0x11, 0x9a, 0x0e, 0xf6, 0x22, 0x33, 0x22, 0xa8,
    0x8e, 0x45, 0xf9, 0x62, 0x1c, 0xa2, 0x49, 0x0b, 0x3a, 0x31, 0xfc, 0xe2, 0x84, 0xd3, 0x5c, 0x9d,
    0x6b, 0x87, 0x0a, 0xad, 0xc7, 0xe5, 0x7f, 0xa3, 0x81, 0x78, 0x5d, 0x6b, 0xdb, 0xfb, 0x7c, 0xd9,
    0x51, 0x47, 0x4b, 0x2f, 0x89, 0x23, 0xa1, 0x04, 0x62, 0xf9, 0x81, 0x60, 0x0b, 0x97, 0x15, 0x57,
    0xba, 0x0a, 0x7d, 0xdb, 0xa9, 0x16, 0x6a, 0x72, 0xd5, 0x12, 0x51, 0xa7, 0xfe, 0xfd, 0x62, 0xb2,
    0xa2, 0x14, 0xfe, 0x3b, 0x50, 0xc1, 0x41, 0x03, 0x20, 0x47, 0xff, 0xdb, 0x43, 0xa7, 0xb8, 0x02,
    0x03, 0x0e, 0x20, 0xc5, 0x00, 0x51, 0xed, 0xcf, 0x0f, 0xb9, 0x21, 0x78, 0xe0, 0x87, 0xfa, 0x5e,
    0x5d, 0x1e, 0x04, 0x5b, 0x38, 0xa0, 0x1b, 0x3a, 0xf4, 0x4c, 0x19, 0x2e, 0x9e, 0xa2, 0x95, 0x81,
    0xe9, 0xc3, 0xbe, 0xfd, 0x1a, 0x10, 0xaa, 0xb2, 0x0c, 0xcd, 0x19, 0xa6, 0xde, 0xc3, 0xca, 0x06,
    0x68, 0xd8, 0x87, 0xe8, 0xf1, 0x86, 0x85, 0x1e, 0x4a, 0x11, 0xe8, 0xc3, 0x79, 0xe0, 0x66, 0xba,
    0xc4, 0xac, 0x99, 0xe8, 0xc6, 0x4b, 0x8b, 0x47, 0xd4, 0x75, 0x79, 0xc9, 0xeb, 0xfc, 0x0f, 0x61,
    0xa7, 0x0d, 0xfa, 0x3f, 0xee, 0xb7, 0x0a, 0x11, 0x7c, 0xb6, 0xfc, 0x6f, 0x3b, 0x01, 0x0b, 0x51,
    0xd1, 0xee, 0xc5, 0xf4, 0xb9, 0x35, 0x98, 0x26, 0x5b, 0x81, 0xe9, 0x1f, 0x65, 0xa2, 0x1c, 0xbf,
    0xf5, 0x39, 0x50, 0x7e, 0xe9, 0x64, 0x7d, 0x5e,
];

#[test]
fn test_integration_auth() {
    let server_secret_params = zkgroup::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();

    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let group_public_params = group_secret_params.get_public_params();

    // Random UID and issueTime
    let uid = zkgroup::TEST_ARRAY_16;
    let redemption_time = 123456u32;

    // SERVER
    // Issue credential
    let randomness = zkgroup::TEST_ARRAY_32_2;
    let auth_credential_response =
        server_secret_params.issue_auth_credential(randomness, uid, redemption_time);

    // CLIENT
    let auth_credential = server_public_params
        .receive_auth_credential(uid, redemption_time, &auth_credential_response)
        .unwrap();

    // Create and decrypt user entry
    let uuid_ciphertext = group_secret_params.encrypt_uuid(uid);
    let plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext).unwrap();
    assert!(plaintext == uid);

    // Create and receive presentation
    let randomness = zkgroup::TEST_ARRAY_32_5;

    let presentation = server_public_params.create_auth_credential_presentation(
        randomness,
        group_secret_params,
        auth_credential,
    );

    let presentation_bytes = &bincode::serialize(&presentation).unwrap();

    //for b in presentation_bytes.iter() {
    //    print!("0x{:02x}, ", b);
    //}
    assert!(AUTH_CREDENTIAL_PRESENTATION_RESULT[..] == presentation_bytes[..]);

    server_secret_params
        .verify_auth_credential_presentation(group_public_params, &presentation)
        .unwrap();

    // test encoding
    // these tests will also discover if the serialized sizes change,
    //   necessitating an update to the LEN constants
    //let mut ccm_bytes = [0u8; zkgroup::common::constants::CLIENT_CREDENTIAL_MANAGER_LEN];
    let mut group_secret_params_bytes = [0u8; zkgroup::common::constants::GROUP_SECRET_PARAMS_LEN];
    let mut server_secret_params_bytes =
        [0u8; zkgroup::common::constants::SERVER_SECRET_PARAMS_LEN];
    let mut group_public_params_bytes = [0u8; zkgroup::common::constants::GROUP_PUBLIC_PARAMS_LEN];
    let mut server_public_params_bytes =
        [0u8; zkgroup::common::constants::SERVER_PUBLIC_PARAMS_LEN];
    let mut auth_credential_response_bytes =
        [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_RESPONSE_LEN];
    let mut auth_credential_bytes = [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_LEN];
    let mut auth_credential_presentation_bytes =
        [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_PRESENTATION_LEN];
    let mut uuid_ciphertext_bytes = [0u8; zkgroup::common::constants::UUID_CIPHERTEXT_LEN];
    let mut uid_bytes = [0u8; zkgroup::common::constants::UUID_LEN];
    let mut randomness_bytes = [0u8; zkgroup::common::constants::RANDOMNESS_LEN];

    //ccm_bytes.copy_from_slice(&bincode::serialize(&client_credential_manager).unwrap());
    group_secret_params_bytes.copy_from_slice(&bincode::serialize(&group_secret_params).unwrap());
    server_secret_params_bytes.copy_from_slice(&bincode::serialize(&server_secret_params).unwrap());
    group_public_params_bytes.copy_from_slice(&bincode::serialize(&group_public_params).unwrap());
    server_public_params_bytes.copy_from_slice(&bincode::serialize(&server_public_params).unwrap());
    auth_credential_response_bytes
        .copy_from_slice(&bincode::serialize(&auth_credential_response).unwrap());
    auth_credential_bytes.copy_from_slice(&bincode::serialize(&auth_credential).unwrap());
    auth_credential_presentation_bytes.copy_from_slice(&bincode::serialize(&presentation).unwrap());
    uuid_ciphertext_bytes.copy_from_slice(&bincode::serialize(&uuid_ciphertext).unwrap());
    uid_bytes.copy_from_slice(&bincode::serialize(&uid).unwrap());
    randomness_bytes.copy_from_slice(&bincode::serialize(&randomness).unwrap());
}

#[test]
fn test_integration_profile() {
    // Random UID and issueTime
    let _uid = zkgroup::TEST_ARRAY_16;

    // SERVER
    let server_secret_params = zkgroup::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();

    // CLIENT
    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let group_public_params = group_secret_params.get_public_params();

    let uid = zkgroup::TEST_ARRAY_16;
    let profile_key =
        zkgroup::profiles::ProfileKey::create(zkgroup::common::constants::TEST_ARRAY_32_1);
    let profile_key_commitment = profile_key.get_commitment(uid);

    // Create context and request
    let randomness = zkgroup::TEST_ARRAY_32_3;

    let context = server_public_params.create_profile_key_credential_request_context(
        randomness,
        uid,
        profile_key,
    );
    let request = context.get_request();

    // SERVER

    let randomness = zkgroup::TEST_ARRAY_32_4;
    let response = server_secret_params
        .issue_profile_key_credential(randomness, &request, uid, profile_key_commitment)
        .unwrap();

    // CLIENT
    // Gets stored profile credential
    let profile_key_credential = server_public_params
        .receive_profile_key_credential(&context, &response)
        .unwrap();

    // Create encrypted UID and profile key
    let uuid_ciphertext = group_secret_params.encrypt_uuid(uid);
    let plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext).unwrap();
    assert!(plaintext == uid);

    let profile_key_ciphertext =
        group_secret_params.encrypt_profile_key(randomness, profile_key, uid);
    let decrypted_profile_key = group_secret_params
        .decrypt_profile_key(profile_key_ciphertext, uid)
        .unwrap();

    assert!(decrypted_profile_key.get_bytes() == profile_key.get_bytes());

    // Create presentation
    let randomness = zkgroup::TEST_ARRAY_32_5;

    let presentation = server_public_params.create_profile_key_credential_presentation(
        randomness,
        group_secret_params,
        profile_key_credential,
    );

    let presentation_bytes = &bincode::serialize(&presentation).unwrap();
    //for b in presentation_bytes.iter() {
    //    print!("0x{:02x}, ", b);
    //}
    assert!(PROFILE_KEY_CREDENTIAL_PRESENTATION_RESULT[..] == presentation_bytes[..]);

    // SERVER

    server_secret_params
        .verify_profile_key_credential_presentation(group_public_params, &presentation)
        .unwrap();

    // test encoding
    // these tests will also discover if the serialized sizes change,
    //   necessitating an update to the LEN constants

    let mut profile_key_commitment_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_COMMITMENT_LEN];
    let mut profile_key_credential_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_LEN];
    let mut profile_key_credential_presentation_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_PRESENTATION_LEN];
    let mut profile_key_credential_request_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_REQUEST_LEN];
    let mut profile_key_credential_request_context_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_REQUEST_CONTEXT_LEN];
    let mut profile_key_credential_response_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_RESPONSE_LEN];

    profile_key_commitment_bytes
        .copy_from_slice(&bincode::serialize(&profile_key_commitment).unwrap());
    profile_key_credential_bytes
        .copy_from_slice(&bincode::serialize(&profile_key_credential).unwrap());
    profile_key_credential_presentation_bytes
        .copy_from_slice(&bincode::serialize(&presentation).unwrap());
    profile_key_credential_request_bytes.copy_from_slice(&bincode::serialize(&request).unwrap());
    profile_key_credential_request_context_bytes
        .copy_from_slice(&bincode::serialize(&context).unwrap());
    profile_key_credential_response_bytes.copy_from_slice(&bincode::serialize(&response).unwrap());
}

#[test]
fn test_client_sigs() {
    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let group_public_params = group_secret_params.get_public_params();
    let randomness = zkgroup::TEST_ARRAY_32_2;
    let message = zkgroup::TEST_ARRAY_32_1;
    let change_signature = group_secret_params.sign(randomness, &message).unwrap();
    assert!(
        &change_signature[..]
            == &[
                0xea, 0x39, 0xf1, 0x68, 0x74, 0x26, 0xea, 0xdd, 0x14, 0x4d, 0x8f, 0xcf, 0xe, 0x33,
                0xc4, 0x3b, 0x1e, 0x27, 0x8d, 0xbb, 0xe0, 0xa6, 0x7c, 0x3e, 0x60, 0xd4, 0xce, 0x53,
                0x1b, 0xcb, 0x54, 0x2, 0xf1, 0x6b, 0x2e, 0x58, 0x7c, 0xa1, 0x91, 0x89, 0xc8, 0x46,
                0x6f, 0xa1, 0xdc, 0xdb, 0x77, 0xae, 0x12, 0xd1, 0xb8, 0x82, 0x87, 0x81, 0x51, 0x2c,
                0xd2, 0x92, 0xd0, 0x91, 0x5a, 0x72, 0xb6, 0x9,
            ][..]
    );
    group_public_params
        .verify_signature(&message, change_signature)
        .unwrap();
}

#[test]
fn test_server_sigs() {
    let server_secret_params =
        zkgroup::api::server_params::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();
    let randomness = zkgroup::TEST_ARRAY_32_2;
    let message = zkgroup::TEST_ARRAY_32_1;
    let change_signature = server_secret_params.sign(randomness, &message).unwrap();
    assert!(
        &change_signature[..]
            == &[
                0x81, 0x9c, 0x59, 0xfc, 0xac, 0xa7, 0x2, 0x3b, 0x13, 0x87, 0x5e, 0xf6, 0x3e, 0xf9,
                0x8d, 0xf3, 0x14, 0xde, 0x2a, 0x6a, 0x56, 0xd3, 0x14, 0xf6, 0x3c, 0xb9, 0x8c, 0x23,
                0x4b, 0x55, 0xf5, 0x6, 0xaf, 0xf6, 0x47, 0x5d, 0x29, 0x57, 0x89, 0xc6, 0x6a, 0x11,
                0xcd, 0xde, 0xc1, 0x60, 0x2e, 0xf1, 0xc4, 0xa2, 0x44, 0x14, 0x16, 0x8f, 0xe9, 0xba,
                0x10, 0x36, 0xba, 0x28, 0x6b, 0x47, 0xea, 0x7,
            ][..]
    );
    server_public_params
        .verify_signature(&message, change_signature)
        .unwrap();
}

#[test]
fn test_blob_encryption() {
    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let randomness = zkgroup::TEST_ARRAY_32_2;

    let plaintext_vec = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19,
    ];

    let ciphertext_vec = vec![
        0xc0, 0x9c, 0x16, 0x75, 0x4b, 0x32, 0x86, 0x7f, 0xd5, 0x11, 0x9d, 0x18, 0x81, 0xd6, 0x2e,
        0x7c, 0x96, 0x7f, 0x6e, 0x3a, 0x8a, 0xf5, 0xf0, 0x9a, 0xc8, 0x4f, 0x7b, 0x74, 0xfc, 0xc6,
        0xd0, 0xe4, 0xd5, 0x9c, 0x9f, 0x4a, 0x17, 0x5e, 0x0f, 0x48, 0x9c, 0x47, 0xe4, 0x81, 0xf1,
    ];

    let calc_ciphertext_vec = group_secret_params
        .encrypt_blob(randomness, &plaintext_vec)
        .unwrap();
    let calc_plaintext_vec = group_secret_params
        .decrypt_blob(&calc_ciphertext_vec)
        .unwrap();
    assert!(calc_plaintext_vec == plaintext_vec);
    assert!(calc_ciphertext_vec == ciphertext_vec);
    //for b in calc_ciphertext_vec.iter() {
    //    print!("0x{:02x}, ", b);
    //}
}