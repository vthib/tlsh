// Those tests comes from tlshc: <https://github.com/avast/tlshc>
// See <https://github.com/avast/tlshc/blob/bb91fef822/tests/test_vectors.cpp>

use super::misc::check;

#[test]
fn test_vector_1() {
    let str1 = b"The quick brown fox jumps over the lazy dog.The quick brown fox jumps over \
                       the lazy dog.The quick brown fox jumps over the lazy dog.The quick brown \
                       fox jumps over the lazy dog.The quick brown fox jumps over the lazy \
                       dog.The quick brown fox jumps over the lazy dog";

    check(
        str1,
        b"T1F7D0024A251C5294648A1888438D98B292C8C51161211421643460022908221DCD8551",
    );
}

#[test]
fn test_vector_2() {
    let str1 = b"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Vivamus ac leo pretium \
        faucibus. Nullam sapien sem, ornare ac, nonummy non, lobortis a enim. Ut tempus purus at \
        lorem. Nulla accumsan, elit sit amet varius semper, nulla mauris mollis quam, tempor \
        suscipit diam nulla vel leo. Curabitur sagittis hendrerit ante. Donec ipsum massa, \
        ullamcorper in, auctor et, scelerisque sed, est. Duis viverra diam non justo. Sed elit \
        dui, pellentesque a, faucibus vel, interdum nec, diam. Class aptent taciti sociosqu ad \
        litora torquent per conubia nostra, per inceptos hymenaeos. Cum sociis natoque penatibus \
        et magnis dis parturient montes, nascetur ridiculus mus. Integer rutrum, orci vestibulum \
        ullamcorper ultricies, lacus quam ultricies odio, vitae placerat pede sem sit amet enim. \
        Duis sapien nunc, commodo et, interdum suscipit, sollicitudin et, dolor. Praesent in \
        mauris eu tortor porttitor accumsan. In convallis. Fusce tellus odio, dapibus id \
        fermentum quis, suscipit id erat. Aliquam erat volutpat. Ut tempus purus at lorem. Nulla \
        non arcu lacinia neque faucibus fringilla. Aliquam id dolor.";

    check(
        str1,
        b"T1BA11B9370D7A075140411376AB64CFAFF71860042A52BFA94CF0FB1FB197E648362268",
    );
}

#[test]
fn test_vector_3() {
    let str1 = b"This text is too short for tlsh by exactly 1 byte";

    check(str1, b"");
}

#[test]
fn test_vector_4() {
    let str1 = b"This text is too short for conservative tlsh, but enough for the others";

    check(
        str1,
        b"T1CDA00241BFCB83B3E0D60948133F2495D35CD5E545A3E224AE81555945131B6467E3D6",
    );
}
