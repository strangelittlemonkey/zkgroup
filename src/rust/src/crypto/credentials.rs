//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

#![allow(non_snake_case)]

use crate::common::constants::*;
use crate::common::simple_types::*;
use crate::crypto::profile_credential_request;
use crate::crypto::uid_encryption;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use serde::{Deserialize, Serialize};
use sha2::Sha512;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemParameters {
    pub(crate) G_w: RistrettoPoint,
    pub(crate) G_wprime: RistrettoPoint,
    pub(crate) G_x0: RistrettoPoint,
    pub(crate) G_x1: RistrettoPoint,
    pub(crate) G_yi: [RistrettoPoint; MAX_CRED_ATTRIBUTES],
    pub(crate) G_mi: [RistrettoPoint; MAX_CRED_ATTRIBUTES],
    pub(crate) G_V: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPair {
    // private
    pub(crate) w: Scalar,
    pub(crate) wprime: Scalar,
    pub(crate) W: RistrettoPoint,
    pub(crate) x0: Scalar,
    pub(crate) x1: Scalar,
    pub(crate) yi: [Scalar; MAX_CRED_ATTRIBUTES],

    // public
    pub(crate) C_W: RistrettoPoint,
    pub(crate) X: RistrettoPoint,
    pub(crate) Yi: [RistrettoPoint; MAX_CRED_ATTRIBUTES],
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    pub(crate) C_W: RistrettoPoint,
    pub(crate) X: RistrettoPoint,
    pub(crate) Yi: [RistrettoPoint; MAX_CRED_ATTRIBUTES],
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) V: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) V: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlindedProfileCredentialWithSecretNonce {
    pub(crate) rprime: Scalar,
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) E_S1: RistrettoPoint,
    pub(crate) E_S2: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlindedProfileCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) E_S1: RistrettoPoint,
    pub(crate) E_S2: RistrettoPoint,
}

pub(crate) fn convert_to_points(
    uid_bytes: UidBytes,
    redemption_time: RedemptionTime,
) -> Vec<RistrettoPoint> {
    let uid_struct = uid_encryption::UidStruct::new(uid_bytes);
    convert_to_points_uid_struct(uid_struct, redemption_time)
}

pub(crate) fn convert_to_points_uid_struct(
    uid_struct: uid_encryption::UidStruct,
    redemption_time: RedemptionTime,
) -> Vec<RistrettoPoint> {
    let system = SystemParameters::get_hardcoded();
    let redemption_time_scalar = encode_redemption_time(redemption_time);
    vec![
        uid_struct.M1,
        uid_struct.M2,
        uid_struct.m3 * system.G_mi[2],
        redemption_time_scalar * system.G_mi[3],
    ]
}

impl SystemParameters {
    pub fn generate() -> Self {
        let G_w = RistrettoPoint::hash_from_bytes::<Sha512>(b"Signal_ZKGroup_Mac_Const_G_w");
        let G_wprime =
            RistrettoPoint::hash_from_bytes::<Sha512>(b"Signal_ZKGroup_Mac_Const_G_wprime");

        let G_x0 = RistrettoPoint::hash_from_bytes::<Sha512>(b"Signal_ZKGroup_Mac_Const_G_x0");
        let G_x1 = RistrettoPoint::hash_from_bytes::<Sha512>(b"Signal_ZKGroup_Mac_Const_G_x1");

        let mut G_yi: [RistrettoPoint; MAX_CRED_ATTRIBUTES] = Default::default();
        let mut G_mi: [RistrettoPoint; MAX_CRED_ATTRIBUTES] = Default::default();
        for i in 0..MAX_CRED_ATTRIBUTES {
            G_yi[i] = RistrettoPoint::hash_from_bytes::<Sha512>(
                format!("Signal_ZKGroup_Mac_Const_G_yi_{}", i).as_bytes(),
            );
            G_mi[i] = RistrettoPoint::hash_from_bytes::<Sha512>(
                format!("Signal_ZKGroup_Mac_Const_G_mi_{}", i).as_bytes(),
            );
        }

        let G_V = RistrettoPoint::hash_from_bytes::<Sha512>(b"Signal_ZKGroup_Mac_Const_G_V");

        SystemParameters {
            G_w,
            G_wprime,
            G_x0,
            G_x1,
            G_yi,
            G_mi,
            G_V,
        }
    }

    pub fn get_hardcoded() -> SystemParameters {
        bincode::deserialize::<SystemParameters>(&SystemParameters::SYSTEM_HARDCODED).unwrap()
    }

    const SYSTEM_HARDCODED: [u8; 480] = [
        0x80, 0x99, 0x1b, 0x54, 0xfc, 0x18, 0x29, 0xe, 0x85, 0x1f, 0x37, 0x25, 0x86, 0x89, 0x72,
        0xd8, 0xf, 0x6d, 0x53, 0x57, 0xc0, 0xe, 0x78, 0x89, 0x93, 0xfb, 0x3b, 0x43, 0x2, 0x13,
        0x30, 0x22, 0xb0, 0x27, 0xe4, 0x17, 0xd5, 0xd, 0xb0, 0xbd, 0x12, 0xc5, 0x44, 0xd3, 0x8b,
        0x5a, 0xf9, 0x14, 0x8, 0x3c, 0xc6, 0x44, 0xdb, 0x98, 0xf4, 0xaa, 0xee, 0x8c, 0xeb, 0x2f,
        0x45, 0x6c, 0x69, 0x57, 0x28, 0xad, 0xc3, 0xf2, 0xf8, 0xd8, 0xb0, 0x83, 0xc, 0xa8, 0xc9,
        0x15, 0x86, 0x55, 0xe, 0xa4, 0x2c, 0x62, 0x7e, 0xa5, 0x28, 0x85, 0xbf, 0xa6, 0x72, 0x6,
        0x3a, 0x5, 0xfc, 0x1b, 0x92, 0x47, 0x5e, 0x9d, 0xf5, 0xe, 0xe9, 0x9, 0x21, 0xda, 0xf5,
        0x53, 0x65, 0xb1, 0xe0, 0x4c, 0xf8, 0xa, 0xf0, 0x7f, 0x5a, 0x21, 0x65, 0xd2, 0x1e, 0x87,
        0xb, 0x61, 0xaf, 0x81, 0x8b, 0xf, 0x4f, 0x35, 0x16, 0xb1, 0x48, 0xd5, 0x61, 0xb6, 0xb,
        0x17, 0xc6, 0xc8, 0x41, 0xa, 0x35, 0x81, 0xa1, 0x1c, 0xf6, 0xf1, 0x66, 0x4c, 0x25, 0x89,
        0xec, 0x34, 0xae, 0xa9, 0xd3, 0xd5, 0xe7, 0x6e, 0x73, 0x20, 0xd6, 0x26, 0xbe, 0x35, 0x49,
        0x6b, 0xa5, 0xb0, 0x70, 0x21, 0xdb, 0xc5, 0xb8, 0xbb, 0x4b, 0x38, 0xd2, 0xcd, 0x28, 0xe5,
        0x4, 0xca, 0xd8, 0x16, 0x50, 0xaa, 0xbc, 0xed, 0xd, 0xa9, 0xa1, 0x1d, 0x84, 0x7f, 0xc6,
        0x3, 0xf1, 0xb5, 0x15, 0x15, 0x9e, 0xd5, 0xe1, 0x63, 0x77, 0x8c, 0xb3, 0x7f, 0x85, 0x22,
        0x68, 0x2e, 0x63, 0x1f, 0x12, 0xc4, 0x3e, 0xfd, 0x31, 0xdc, 0x38, 0xd9, 0xd, 0x3b, 0x2,
        0x82, 0xcf, 0xa2, 0xce, 0x46, 0xbe, 0xf6, 0x2c, 0x59, 0x6d, 0xd7, 0x37, 0xbc, 0xcd, 0xe6,
        0x8d, 0xfd, 0x44, 0x59, 0xaa, 0x48, 0x34, 0x9b, 0x67, 0x71, 0x88, 0x89, 0x84, 0xf, 0x93,
        0x57, 0x90, 0xca, 0x13, 0xe9, 0xa0, 0x85, 0xa3, 0x74, 0x95, 0x14, 0x81, 0x36, 0x2a, 0xb3,
        0xab, 0xfe, 0x3b, 0xc, 0xed, 0x3f, 0x3b, 0x8d, 0xfb, 0x6f, 0x1c, 0xb9, 0x44, 0x6a, 0x25,
        0xaa, 0x3b, 0x6f, 0x68, 0x22, 0xd, 0x49, 0xf7, 0x30, 0xed, 0xe5, 0x99, 0xdc, 0x38, 0xb,
        0xc6, 0xeb, 0x9f, 0x3d, 0x2c, 0x1a, 0x37, 0xea, 0x96, 0xea, 0x87, 0x2c, 0x0, 0xb2, 0x6a,
        0x74, 0xa2, 0x8a, 0x19, 0x1a, 0x1c, 0x3b, 0xdb, 0xe3, 0x94, 0xda, 0xa6, 0x1f, 0x57, 0x91,
        0xb9, 0x7d, 0xe8, 0xfa, 0xfd, 0x7a, 0x53, 0x1c, 0x95, 0x59, 0x24, 0xd, 0x7a, 0x4d, 0x71,
        0xca, 0x6b, 0xa, 0x0, 0x35, 0x17, 0x4f, 0xc2, 0xf1, 0x53, 0xde, 0x41, 0x72, 0x8f, 0xb2,
        0x61, 0xb5, 0xd1, 0x1c, 0x56, 0x71, 0x79, 0xdd, 0x39, 0x94, 0xe9, 0xfe, 0xde, 0xa2, 0x56,
        0x82, 0x3c, 0xa9, 0x56, 0x8c, 0xe8, 0x2d, 0xf2, 0x59, 0x0, 0xcf, 0x2, 0xa3, 0xf0, 0xdb,
        0x9c, 0xe0, 0x5e, 0xf9, 0x1b, 0xaf, 0x6f, 0xa1, 0x64, 0xe1, 0x43, 0xe8, 0xde, 0x6b, 0xc9,
        0xdd, 0xbc, 0x42, 0x8f, 0xf2, 0x5a, 0x29, 0xa0, 0xd, 0x65, 0x10, 0xc, 0xe1, 0x9b, 0x58,
        0x7c, 0x83, 0xf, 0x7e, 0xeb, 0x53, 0x1a, 0x6d, 0xb9, 0xf5, 0x2b, 0x41, 0xda, 0x50, 0x42,
        0x40, 0xb3, 0x4c, 0xe2, 0x17, 0x15, 0x74, 0xac, 0xc3, 0xb0, 0x49, 0xf, 0x65, 0x90, 0x7b,
        0xc5, 0xd8, 0x6b, 0x21, 0x91, 0x19, 0x4b, 0xf2, 0x6d, 0x40, 0x63, 0x4e, 0xfe, 0x49, 0x30,
        0xee, 0x2a, 0x61, 0xb5, 0x7, 0x4d, 0x2d, 0x77, 0x50, 0x72, 0x3f, 0x7a, 0x2, 0x79, 0x1d,
    ];
}

impl KeyPair {
    pub fn generate(randomness: RandomnessBytes) -> Self {
        let system = SystemParameters::get_hardcoded();
        let w = calculate_scalar(b"Signal_ZKGroup_Mac_KeyGen_w", &randomness);
        let W = w * system.G_w;
        let wprime = calculate_scalar(b"Signal_ZKGroup_Mac_KeyGen_wprime", &randomness);
        let mut yi: [Scalar; MAX_CRED_ATTRIBUTES] = Default::default();
        let mut Yi: [RistrettoPoint; MAX_CRED_ATTRIBUTES] = Default::default();
        for i in 0..MAX_CRED_ATTRIBUTES {
            yi[i] = calculate_scalar(
                format!("Signal_ZKGroup_Mac_KeyGen_yi_{}", i).as_bytes(),
                &randomness,
            );
            Yi[i] = yi[i] * system.G_yi[i];
        }
        let x0 = calculate_scalar(b"Signal_ZKGroup_Mac_KeyGen_x0", &randomness);
        let x1 = calculate_scalar(b"Signal_ZKGroup_Mac_KeyGen_x1", &randomness);

        let C_W = (w * system.G_w) + (wprime * system.G_wprime);
        let X = (x0 * system.G_x0) + (x1 * system.G_x1);

        KeyPair {
            w,
            wprime,
            W,
            x0,
            x1,
            yi,
            C_W,
            X,
            Yi,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        PublicKey {
            C_W: self.C_W,
            X: self.X,
            Yi: self.Yi,
        }
    }

    pub fn create_auth_credential(
        &self,
        uid_bytes: UidBytes,
        redemption_time: RedemptionTime,
        randomness: RandomnessBytes,
    ) -> AuthCredential {
        let (t, U, V) = self.credential_core(uid_bytes, redemption_time, randomness);
        AuthCredential { t, U, V }
    }

    fn credential_core(
        &self,
        uid_bytes: UidBytes,
        redemption_time: RedemptionTime,
        randomness: RandomnessBytes,
    ) -> (Scalar, RistrettoPoint, RistrettoPoint) {
        let M = convert_to_points(uid_bytes, redemption_time);
        let t = calculate_scalar(b"Signal_ZKGroup_MAC_Random_t", &randomness);
        let U = calculate_scalar(b"Signal_ZKGroup_Mac_Random_U", &randomness)
            * RISTRETTO_BASEPOINT_POINT;

        let mut V = self.W + (self.x0 + self.x1 * t) * U;
        for i in 0..M.len() {
            V += self.yi[i] * M[i];
        }
        (t, U, V)
    }

    pub fn create_blinded_profile_credential(
        &self,
        uid_bytes: UidBytes,
        redemption_time: RedemptionTime,
        public_key: profile_credential_request::PublicKey,
        ciphertext: profile_credential_request::Ciphertext,
        randomness: RandomnessBytes,
    ) -> BlindedProfileCredentialWithSecretNonce {
        let (t, U, Vprime) = self.credential_core(uid_bytes, redemption_time, randomness);
        let rprime = calculate_scalar(b"Signal_ZKGroup_BlindIssueMac_KeyGen_rprime", &randomness);
        let E_R1 = rprime * RISTRETTO_BASEPOINT_POINT;
        let E_R2 = rprime * public_key.D + Vprime;
        let E_S1 = E_R1 + self.yi[4] * ciphertext.E_D1;
        let E_S2 = E_R2 + self.yi[4] * ciphertext.E_D2;
        BlindedProfileCredentialWithSecretNonce {
            rprime,
            t,
            U,
            E_S1,
            E_S2,
        }
    }
}

impl BlindedProfileCredentialWithSecretNonce {
    pub fn get_blinded_profile_credential(&self) -> BlindedProfileCredential {
        BlindedProfileCredential {
            t: self.t,
            U: self.U,
            E_S1: self.E_S1,
            E_S2: self.E_S2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::proofs;

    #[test]
    fn test_system() {
        //let params = SystemParameters::generate();
        assert!(SystemParameters::generate() == SystemParameters::get_hardcoded());
    }

    #[test]
    fn test_mac() {
        let keypair = KeyPair::generate(TEST_ARRAY_32);

        let uid_bytes = TEST_ARRAY_16;
        let redemption_time = 37;
        let randomness = TEST_ARRAY_32;
        let credential = keypair.create_auth_credential(uid_bytes, redemption_time, randomness);
        let proof = proofs::AuthCredentialIssuanceProof::new(
            keypair,
            credential,
            uid_bytes,
            redemption_time,
            randomness,
        );

        let public_key = keypair.get_public_key();
        proof
            .verify(public_key, credential, uid_bytes, redemption_time)
            .unwrap();

        let keypair_bytes = bincode::serialize(&keypair).unwrap();
        let keypair2 = bincode::deserialize(&keypair_bytes).unwrap();
        assert!(keypair == keypair2);

        assert!(keypair_bytes.len() == 544);

        let public_key_bytes = bincode::serialize(&public_key).unwrap();
        let public_key2 = bincode::deserialize(&public_key_bytes).unwrap();
        assert!(public_key == public_key2);
        assert!(public_key_bytes.len() == 224);

        let mac_bytes = bincode::serialize(&credential).unwrap();

        //println!("mac_bytes = {:#x?}", mac_bytes);
        assert!(
            mac_bytes
                == vec![
                    0x56, 0xac, 0x4e, 0x56, 0xb, 0x22, 0x1a, 0xd0, 0xa8, 0xa1, 0xfe, 0xdd, 0xed,
                    0xfb, 0x26, 0x3d, 0x9c, 0x54, 0x67, 0x4b, 0x18, 0x94, 0x52, 0x2, 0x2, 0x4c,
                    0xea, 0xcb, 0x9e, 0x6, 0x18, 0x6, 0xfc, 0xff, 0x19, 0xdf, 0x46, 0x4f, 0x21,
                    0x1e, 0xfb, 0x70, 0xd9, 0xe1, 0x5b, 0xd, 0x8d, 0x90, 0xcc, 0xce, 0x2f, 0x7c,
                    0x42, 0x23, 0x1e, 0x39, 0x3f, 0x90, 0x8a, 0xe8, 0x2d, 0xcb, 0x99, 0x2, 0xac,
                    0x23, 0x5f, 0x97, 0x9a, 0xfc, 0xee, 0x8f, 0xde, 0xd, 0x6, 0xd, 0xdb, 0x7c,
                    0x83, 0x60, 0xcc, 0x11, 0x1, 0x4c, 0x78, 0x9d, 0x31, 0x4c, 0x2d, 0xd8, 0x92,
                    0xca, 0xd1, 0x66, 0x5b, 0x41,
                ]
        );

        //println!("keypair = {:#x?}", keypair_bytes);
        assert!(
            keypair_bytes
                == vec![
                    0x49, 0x72, 0xd4, 0xf9, 0xb9, 0xa7, 0x94, 0xa4, 0x1c, 0x72, 0x31, 0x6d, 0xed,
                    0xfb, 0xf8, 0x7e, 0x8e, 0x8, 0xed, 0x85, 0xd4, 0x86, 0x60, 0xbb, 0xa7, 0x9a,
                    0xc5, 0xcd, 0x1, 0xfe, 0x65, 0x8, 0x30, 0xd5, 0x30, 0xa6, 0x18, 0x83, 0xf6,
                    0x6f, 0x7f, 0x2f, 0xd8, 0x84, 0xdc, 0x1e, 0x4d, 0xcb, 0x9a, 0x3, 0xdc, 0xf,
                    0xef, 0x56, 0x15, 0x71, 0x11, 0x20, 0xe, 0x76, 0x80, 0x72, 0x3, 0xe, 0x86,
                    0x82, 0x82, 0xea, 0xff, 0xdf, 0xa1, 0x2c, 0xa4, 0x13, 0x60, 0xe5, 0xa5, 0xfc,
                    0xf5, 0x36, 0x90, 0xae, 0x45, 0xc1, 0x2a, 0xb, 0x36, 0xde, 0x7, 0xd3, 0xd3,
                    0xb7, 0x9a, 0x3b, 0x11, 0x44, 0xf5, 0xe8, 0xd8, 0x9, 0x65, 0xfd, 0x3c, 0x5d,
                    0x75, 0x3c, 0x1a, 0x31, 0x1, 0x22, 0x28, 0xf1, 0xab, 0x61, 0x9c, 0x9b, 0x5a,
                    0x74, 0xd5, 0x9d, 0xb9, 0xf5, 0xb2, 0x8e, 0x2d, 0xfe, 0xb0, 0xc, 0xb1, 0x8b,
                    0x56, 0xde, 0x77, 0xf, 0x78, 0xdb, 0x54, 0xad, 0xf, 0xf8, 0xb6, 0x42, 0xc7,
                    0xb6, 0xb2, 0xdd, 0x72, 0xb9, 0x94, 0x93, 0xe8, 0xcd, 0x4a, 0xa7, 0xd5, 0x2f,
                    0xc0, 0x9e, 0x38, 0x9, 0x1d, 0x88, 0x5b, 0xb7, 0x86, 0x2d, 0xb, 0xca, 0x39,
                    0xd1, 0x7e, 0x15, 0x3e, 0x3, 0x94, 0x3, 0xfc, 0x7a, 0x2, 0xa6, 0x55, 0xe4,
                    0xad, 0x14, 0xbf, 0x9c, 0x81, 0xb6, 0xb2, 0xe6, 0xd2, 0x9, 0xc3, 0xec, 0xd2,
                    0x86, 0x80, 0x8c, 0xac, 0x36, 0xfe, 0xd8, 0xf3, 0x3c, 0x88, 0x7c, 0x86, 0xbc,
                    0xcd, 0xac, 0x80, 0xc0, 0xe6, 0x1e, 0x33, 0xa5, 0xc7, 0x7, 0x39, 0xd, 0x70,
                    0x71, 0x85, 0x1, 0xe7, 0xa5, 0xae, 0x28, 0xa0, 0xbc, 0x33, 0x17, 0x6c, 0xbe,
                    0xe8, 0xd8, 0xc4, 0x9e, 0x2f, 0x11, 0xa5, 0x30, 0x7, 0xc2, 0xe8, 0xa5, 0x8f,
                    0x5f, 0x3f, 0xe4, 0x73, 0x7d, 0x68, 0x41, 0xdb, 0x7, 0x56, 0x66, 0xc4, 0x8d,
                    0x83, 0x79, 0x5c, 0x1f, 0x33, 0x30, 0x27, 0x1c, 0x51, 0xc4, 0xd4, 0xf6, 0x3d,
                    0x25, 0xbf, 0x34, 0xb7, 0x1d, 0xf4, 0x34, 0x95, 0x38, 0x45, 0x60, 0xce, 0xea,
                    0xc9, 0x9, 0x7, 0xe0, 0x19, 0xaa, 0x99, 0xfa, 0xc, 0x3f, 0x4a, 0xec, 0x20,
                    0xee, 0x7c, 0xf1, 0x1d, 0xb7, 0xc4, 0x27, 0xe2, 0xa0, 0x9d, 0x2, 0x7a, 0x37,
                    0xd8, 0xf6, 0x93, 0x43, 0x27, 0xfb, 0xe9, 0x6, 0x4a, 0xe2, 0x6b, 0x2d, 0x71,
                    0x96, 0x3f, 0x8f, 0x38, 0xec, 0x25, 0x2d, 0xcd, 0xc7, 0x2d, 0xcd, 0x86, 0xe7,
                    0x6b, 0xef, 0x79, 0x39, 0x6, 0x4d, 0x62, 0xa0, 0x5c, 0xd1, 0x3a, 0x4b, 0x6f,
                    0x72, 0x7e, 0xea, 0x7b, 0x63, 0x42, 0x33, 0x22, 0x29, 0x7a, 0xb7, 0x6d, 0xf,
                    0xd9, 0xed, 0x22, 0xf5, 0xea, 0xf9, 0x86, 0x48, 0xf7, 0xed, 0xbf, 0x7f, 0xd7,
                    0x28, 0x22, 0x3e, 0xab, 0xc2, 0x3c, 0x4b, 0x28, 0x1d, 0x2b, 0xba, 0x3d, 0xb3,
                    0x93, 0xcb, 0xae, 0x3a, 0x0, 0xa2, 0x39, 0x9e, 0x63, 0x4a, 0xf3, 0x4, 0x52,
                    0x62, 0xb9, 0xdd, 0x6f, 0x67, 0xb0, 0x57, 0x4d, 0x45, 0x90, 0x57, 0xa4, 0x7c,
                    0x5a, 0x85, 0x63, 0xb1, 0xa9, 0xe2, 0xfd, 0xb4, 0x81, 0x54, 0x9a, 0xaf, 0x91,
                    0xe7, 0xc5, 0x4f, 0x49, 0xed, 0x64, 0x57, 0x9b, 0x37, 0x87, 0x82, 0x1d, 0x5f,
                    0xbc, 0xb2, 0x7b, 0xa9, 0xb, 0x22, 0xa2, 0xed, 0x2b, 0x50, 0x4d, 0x1a, 0x6e,
                    0xa6, 0x39, 0xe0, 0x34, 0xa9, 0x4e, 0x8c, 0x2a, 0x2c, 0xd2, 0x57, 0xb7, 0xbe,
                    0x89, 0x22, 0xbb, 0xda, 0x47, 0xeb, 0x6a, 0xcf, 0xeb, 0x7, 0xd2, 0x2d, 0xfc,
                    0x23, 0xe2, 0xd0, 0xfd, 0x3d, 0x43, 0x62, 0x6, 0xc8, 0x36, 0x27, 0x25, 0xf3,
                    0x97, 0x1d, 0x76, 0x4b, 0x7b, 0x61, 0x81, 0x92, 0x78, 0xa, 0x5b, 0x67, 0x41,
                    0xf8, 0xcf, 0x1e, 0x6d, 0x68, 0x48, 0x69, 0x6a, 0xc6, 0xd0, 0x3a, 0x2, 0x58,
                    0xd1, 0x0, 0x18, 0x54, 0x8c, 0xba, 0x63, 0x26, 0x87, 0x9b, 0xc5, 0x45, 0xdb,
                    0x7c, 0x4f, 0x99, 0x61, 0xa2, 0x6a, 0xb0, 0x88, 0x94, 0xcd, 0x5e,
                ]
        );
    }

    #[test]
    fn test_blind_issue_mac() {
        /*
        let keypair = KeyPair::generate(TEST_ARRAY_32);
        let uid = UidStruct::new(TEST_ARRAY_16);
        let attributes: [PointOrScalar; 4] = [
            PointOrScalar::P(uid.M1),
            PointOrScalar::P(uid.M2),
            PointOrScalar::S(uid.m3),
            PointOrScalar::S(Scalar::from_bytes_mod_order(TEST_ARRAY_32)),
        ];
        let randomness = TEST_ARRAY_32;


        let (mac, proof) = keypair.create_mac(&attributes, randomness);

        keypair.verify_mac(&attributes, mac).unwrap();

        let public_key = keypair.get_public_key();
        public_key
            .verify_mac_with_proof(&attributes, mac, &proof)
            .unwrap();

        let keypair_bytes = bincode::serialize(&keypair).unwrap();
        let keypair2 = bincode::deserialize(&keypair_bytes).unwrap();
        assert!(keypair == keypair2);

        assert!(keypair_bytes.len() == 544);

        let public_key_bytes = bincode::serialize(&public_key).unwrap();
        let public_key2 = bincode::deserialize(&public_key_bytes).unwrap();
        assert!(public_key == public_key2);
        assert!(public_key_bytes.len() == 224);

        //let mac_bytes = bincode::serialize(&mac).unwrap();
        //
        */
    }
}