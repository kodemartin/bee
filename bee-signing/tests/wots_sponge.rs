// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(deprecated)]

use bee_crypto::ternary::sponge::{CurlP27, CurlP81, Kerl, Sponge};
use bee_signing::ternary::{
    seed::Seed,
    wots::{Error as WotsError, WotsSecurityLevel, WotsSpongePrivateKeyGeneratorBuilder},
    PrivateKey, PrivateKeyGenerator, PublicKey, RecoverableSignature,
};
use bee_ternary::{T1B1Buf, TryteBuf};

use std::str::FromStr;

const SEED: &str = "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN";
const MESSAGE: &str = "CHXHLHQLOPYP9NSUXTMWWABIBSBLUFXFRNWOZXJPVJPBCIDI99YBSCFYILCHPXHTSEYSYWIGQFERCRVDD";

#[test]
fn generator_missing_security_level() {
    assert_eq!(
        WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default().build().err(),
        Some(WotsError::MissingSecurityLevel)
    );
}

#[test]
fn generator_valid() {
    let security_levels = vec![
        WotsSecurityLevel::Low,
        WotsSecurityLevel::Medium,
        WotsSecurityLevel::High,
    ];
    for security in security_levels {
        assert!(
            WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
                .with_security_level(security)
                .build()
                .is_ok(),
        );
    }
}

fn roundtrip<S: Sponge + Default>() {
    let message_trits = TryteBuf::try_from_str(MESSAGE).unwrap().as_trits().encode::<T1B1Buf>();
    let seed = Seed::from_str(SEED).unwrap();
    let security_levels = vec![
        WotsSecurityLevel::Low,
        WotsSecurityLevel::Medium,
        WotsSecurityLevel::High,
    ];
    for security in security_levels {
        for index in 0..3 {
            let private_key_generator = WotsSpongePrivateKeyGeneratorBuilder::<S>::default()
                .with_security_level(security)
                .build()
                .unwrap();
            let mut private_key = private_key_generator.generate_from_seed(&seed, index).unwrap();
            let public_key = private_key.generate_public_key().unwrap();
            let signature = private_key.sign(&message_trits).unwrap();
            let recovered_public_key = signature.recover_public_key(&message_trits).unwrap();

            assert_eq!(public_key.as_trits(), recovered_public_key.as_trits());
            assert!(public_key.verify(&message_trits, &signature).unwrap());
        }
    }
}

#[test]
fn kerl_roundtrip() {
    roundtrip::<Kerl>();
}

#[test]
fn curl27_roundtrip() {
    roundtrip::<CurlP27>();
}

#[test]
fn curl81_roundtrip() {
    roundtrip::<CurlP81>();
}

#[test]
fn example() {
    let key_trytes = "D9DWVXXXMGBR9BKHQMMRTQIQROKTLOZNNYHHETDLHVE9FUIBGLEVSTHMJHNHXRRYWHBLNUICBOQHVGBRDSAJZKOL9EQNXGHCETSGW9LTGJPMKD9DEFITRUDVUUSKR9BMMAABVJIKICTLEGBSDMHGGOEALGWBZCF9YCWUANXAZOZJFQARYCKNOJTCXBYEWENKABGSUQXLOM9ZNEDVZQMUPWROQVQXTHLDUKMRULEZDOW9MEXSELWIKHYTCXQNWYIRLNQWUEOBPBQZTHNMRSMNIRQ9BKDHPDGN9VRZEBXKTFCCXOETJKX9YYJDB9HGLCOHMPOXRXBBV9YDSLYPBVABTNOWKXLSTGZAA9EXHLLGWIISYOSQQKYYOXSJDXVSLHDASEXDYJTFRRHLRMKLHZESDGTDGLUNLOWIETHXWEWPDCJRGMKFMXVNPLBGRYMAXTTAILUGAFUIXGTDP9JIVZFMEJKZRIYPLQNPSPQGZWZJZQXUVTJGRMCCJLNOLFZUCSEPNNNSYKLHJZH9YXTTTEPMDBWGXYWAMAMIDXYYSUBJKFAIH9XXBGBXMSBPJZGIDSELVW9PCTDYNKRCCZQIUIRIBNVWY9EFTZPJFLOF9GRJJQEPNOXHJHZE9KJXFX9TZZHNIVAQUSHAWTDQVGNMRMNRCKRZWCKWBBWKERTTBYKINHRYVTLVHSEDJTXVQUPBYJEQ9FFEEZOPG9HJCLPLOI9RKDDTCS9QWOZMYQJBKHRLHLCMUJODPLZFVFBINPLBLDI9EHNECSMTRWIIPVNRAWARRJSXMXCDSIENRC9MFIY9CYVLUOIWCONKRBVDRENXLXQKOGGLXWPSMQJUKUDRHBWVDQEWWSJKREESCIBIQNIWWPPWNLCBD9WYXOXCAHWLCIHTFSSNMAFPUUKZQEKWZKOYVIYSCYTOKNOSEPWPTVQBUUDUVDECYJWJQRYOPIKDLTHEAAJYQWFNMCGXRHBJTIPCPVGBVYOQTAWPHQBZEEYVNBNMHVRJDFBT9OFYRPUEIHBQIYB9TUZUMLLZAYVZKEBTQGKJPXPACTOKIG9DMUABKEFROB9FRM99JSPXZQDXTTGPPKUYMLLA9LEUFZYMWHDXTVOAUTJLC9SMEBTJMZPWNKGZTZZOYLOC9JGOQUOYYOSUSS9GNYNLROXMPFYIXPA9UOKATDFVJIGDGLMNGBCORUQMTFAQI9PDUUDRPBCRUJW9ZDHXDVGIIZLAKZQBQXZJZFZZDKKMTBMPYFEKJB9RHRZYJJMQTFLIYRHGUWEJWDD9K9INENNZPET9AJPCBPIBOZPHQTVDSCUGHFWZIDAOHXDCOUTANGUNBUHWGHXODGZCMANRSNUKFXXJLUGOZVZMFZTVSUBCGVMJAEQJFKC9HYMVNOULDFUCCXYBAGGXXDVE9ZETYAGJFXNE9DKVKDLGLYXHTFJDWKJZXUNUUVHKYRHANXJVFWVNFYWLEPMFFMMVIYRDUWSKOHDH9VFLMAKVPOVBH9GOS9TOUXDTMZPMTSCLPRUEPSNWAMVIONKWMUJYQYDZKSNCURUCARCGQUPTTVDBHQFPXMDYODFFAFTQVCJGXPZUZEZVCVVONHH9ZHLVYKUORZBGVBODHWMDMZCCQYZKQCZHHMQNPEVKTFAXFGNKCKJXQWSYKIFPVQFCPBTSCNQVHDTXBWWBGQZ9PAQCQFNSKQCAHIHOGYNVBENKJNWCBRFAU9H9ZDORYB9HDVIZOCUBP9PLCMXPEIOIOFPYWWLYWHVUJVMQCOGIMU99X9QRNURAAMROSXVYRJBK9LQCGREWFNLSPOXRGAINPAMOMZJMEAELKHJRQFLKYTEDEFGK9CNQUXPXHO9LUMZITY9MSESJJN9ZSZLNMIKLFABBKFMLGNRT9YLHUHBYMPPFXMWDPTM9T9GSBOYXSAGCRTPB9WVBOBA9QGTRSSJJUAKPZAOXNGSENLZEWOLRRFYVEGWOQJBGMXQRMDOQMLVIJKFUW9KCQHNZ9G9OBMCJSQIFPXDNQTOLFBHDHDQVBAZPBDW9SKV9TUGHW9JLPBCGLOWCFFYUCCDJARXMMZVXPIJKWTNKTXXRPFGCJUQVOASZYHDPAXFWTEQDCCZRQZCNHUXBEMLAENNPZHTID9PUCM9AQMJSJBXVH9TWUEUVRWZRKHAMPBKYYMRPXDNUWZHBCO9NQJ99MZAIBTXOFU9RDCKKIDOEACSPMIYPUFXXGBYCIVV9AHTLPPNGWJHOQBPNFKYKGCTFFEQLRPYEPMIOXLBYTKZ9YZPWXNBZVUBDAWHBDPIFCTGICEHQNTMT9AAUTI9KXAVKEZEWGFZSPVXHOVKNDXKINGSMSWGMUXHGTOAKHASIAIMEDPTPFYVE9BBSIB9OL9YBCEDHEVPCPJHTUCXFCFASRFZIDKZRSQJVEHSTKHVQXFYJFCOJDPDSQFAPEWVURE9FBSORQEXYBANRQGAWLNVPBXVEBHNULWKJJKPCXCP9QXNWVATDNRBHVWXRIQCNQMDJHMTIIHSHEYXJQVWTDASLSSISNTYMTGJUVHRPVXOZFOMYEIPK9DSLZNGEPPZDCCFGJVBJEHVSOPZANIJYYFZMSAAHNBEXJ9ONTQWMHAFKYTGUNLR9GPHWODRSIDYSJGCEOFKETUKMOBFWBXYXUVD9OAQPDDGAGXICQAVUGXGKPPZYZFNGLIYNEKTUCCWXZQLWZXDKIMQCBMCZIPN9JW9AXFNKMJMEXEJPRDJXVGLJSYUZXUPWK9E9UYBXIZXCGBRCLLWGI9RHVVUTSLYMIREQEPSMIKSG9FTOJWVECSXYERNKMSV9JZXKSTRFVYRENPUOALHGYGIRNMYBEHCZLVMPDEWEFACCJUHRJFPIX9GJZWNZLOLRIL9QJUVEENRRQEBF9OWMXEBU9APHLCTYOKBWFHIWARHPWSZWRWDPGFOGHEBHXHGIODOAG9FDMXCUNVPBPETXQSWHKLEKRHIPDZ9OYENTVRQTMNKGGLVDDIPMLFPFAKQGXQJYHRUXQNWH9STRJE9DEEPMRVBUWAZWWJUNUGBKX9ULPLYAOLKTPNLIAVDWWHK9NPFFL9QCZMXGVSWJWWC9JIJRZWJSDXSJEUYESNLINGBFBTEYMURQP9RRBD9XSNILEFSDXK9ZPRPLYJHYCWAKVOSVEOUFHBSVQOWBEKNVWKMICTUXNPROPPWGYRAPGLHFCMVWCLWEVKCASBGEGYOMJJLMDXDZPNADTSFPOFKRQKQSEULQTTJARXHKTDMNZEIVSZPIKFGTOMDFQEQEBOVGKGNC9ZMVF9AEAHEXZ9CXUAWTSGNPPUJQRREXAFJCLISFHQJUWFDNETQUCMXTEWNJUSAKRNXHA9RFYUDFYSNOFTLLYJH9YTOPWPITUBTFFGWAFXQ9CYORFC9XSUQRWEJXPWWF9IGDIYGTPLVOCFLLCHOZADUBFXQSHMXCURSCVHIJSVBBDYCYCLEEFRLIKMYHZJABDKFMWXOAVLYHKTYTWEXFUPKCPDCMTZYQRVFLGJEYFUXRDYNAJE9ZPA9XPDZCTOWZBANDDTSLI9OWYOYSGZGXWHNBQDMDGPZCMBERFPYQSBHQFFXQCK9YPBPBIOATPYJZPLJTDVYFKZFJCGSNLXJKSOYTHLJIAGLPBNLQVPBPTDSDVFNXXQRQQLNCBPAHOKWKRZFOCA9MAARTPFNYNOESMDTML99HHOIODAHDOYCULH9YNFUVWCLYECPHGVBXYVFMLYWXJU9ZTQXWBEPDWAOTMPADNUESWPQRXLJVLAKKMBRYQYLSLLBPFMZJPTKABQHMYBQAGQIYHIM9XSOEAFAYBHJLCVZIDFMNIQYXTFQCKXWWDKKKFKJWNT9DTTWLPNSOXBRVPPJMQUP9NEYJRNUWUODWZWTYHVTPSKLBPVRCERSMHFVVWJEUSHNXSYXFIBKHYRBSPAJPJ9V9UATWMVRXSCP9UVNM9ANXIXEKSXMXOXPOBHFLKIMHCEVJGAFWYFSMVNLYUSVBUHNGQNZMXIXBPUPDWAHDSIDNTTMHGEG9JFVDXGQVZMOWYRXUJWRIGTRAEVZGQDAXGILD9IQQUWAM9LHNHHRYXIYOZBDNEVORRDDOKICQCDMTDJUPIWZLEYCOOFSYOJLVKXLSBOTGAHXMLQYQEBEP9PZ9KKUOYIGECFTXXNBJIOJXMHCPJ9HOTLW9PSOHFAL9UTNUXVKF9IWMNJ9FITOC9FIYWQFSXPWAKYMIUHBK99DKLCTZUANNGVEEWEVVKMNQPQVVLG9ICNTACUTUPZYMYZKZTVEEVUGFWJGGWITRLJA9NHFYDVHGQLOBOUJHHCAHFRCFZIEXNZRLZFBVQUXOIKYKQR9JAQAZLAEYTBSREGMVEILTOWFXPHMNGGCTGLECZB";
    let entropy =
        TryteBuf::try_from_str("CEFLDDLMF9TO9ZLLTYXIPVFIJKAOFRIQLGNYIDZCTDYSWMNXPYNGFAKHQDY9ABGGQZHEFTXKWKWZXEIUD")
            .unwrap()
            .as_trits()
            .encode::<T1B1Buf>();
    let key = TryteBuf::try_from_str(key_trytes)
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>();
    let private_key_generator = WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
        .with_security_level(WotsSecurityLevel::Medium)
        .build()
        .unwrap();
    let generated_key = private_key_generator.generate_from_entropy(&entropy).unwrap();

    assert_eq!(&key.as_slice(), &generated_key.as_trits());
}

#[test]
fn invalid_entropy_length() {
    let entropy = TryteBuf::try_from_str("CEFLDDLMF9TO9ZLLTYXINXPYNGFAKHQDY9ABGGQZHEFTXKWKWZXEIUD")
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>();
    let private_key_generator = WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
        .with_security_level(WotsSecurityLevel::Medium)
        .build()
        .unwrap();

    assert_eq!(
        private_key_generator.generate_from_entropy(&entropy).err(),
        Some(WotsError::InvalidEntropyLength(entropy.len()))
    );
}

#[test]
fn non_null_last_entropy_trit() {
    let entropy =
        TryteBuf::try_from_str("CEFLDDLMF9TO9ZLLTYXIPVFIJKAOFRIQLGNYIDZCTDYSWMNXPYNGFAKHQDY9ABGGQZHEFTXKWKWZXEIUS")
            .unwrap()
            .as_trits()
            .encode::<T1B1Buf>();
    let private_key_generator = WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
        .with_security_level(WotsSecurityLevel::Medium)
        .build()
        .unwrap();

    assert_eq!(
        private_key_generator.generate_from_entropy(&entropy).err(),
        Some(WotsError::NonNullEntropyLastTrit)
    );
}
