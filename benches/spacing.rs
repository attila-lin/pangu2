#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| pangu2::spacing(""));
}

#[bench]
fn simple(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("abcあ123"));
}

#[bench]
fn simple_spaced(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("abc あ 123"));
}

#[bench]
fn sentence(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("新八的構造成分有95%是眼鏡、3%是水、2%是垃圾"));
}

#[bench]
fn sentence_spaced(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("新八的構造成分有 95% 是眼鏡、3% 是水、2% 是垃圾"));
}

#[bench]
fn paragraph(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("逻辑学的领域是研究证明，即对特定命题的真伪性进行不容置疑的论证。有关逻辑学在计算机科学中核心作用的书卷汗牛充栋。Manna和Waldinger称之为「计算机科学的微积分」，而Halpern的论文On the Unusual Effectiveness of Logic in Computer Science中则收录了大量逻辑学为计算机科学提供的洞察力和至关重要的工具。的确，他们发现：「事实上，逻辑学在计算机科学中远比在数学中更加有效。这相当引人注目，特别是由于过去一百年来，逻辑学发展的动力大都来自于数学。」"));
}

#[bench]
fn paragraph_spaced(b: &mut Bencher) {
    b.iter(|| pangu2::spacing("逻辑学的领域是研究证明，即对特定命题的真伪性进行不容置疑的论证。有关逻辑学在计算机科学中核心作用的书卷汗牛充栋。Manna 和 Waldinger 称之为「计算机科学的微积分」，而 Halpern 的论文 On the Unusual Effectiveness of Logic in Computer Science 中则收录了大量逻辑学为计算机科学提供的洞察力和至关重要的工具。的确，他们发现：「事实上，逻辑学在计算机科学中远比在数学中更加有效。这相当引人注目，特别是由于过去一百年来，逻辑学发展的动力大都来自于数学。」"));
}
