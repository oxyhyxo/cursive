extern crate cursive;

use cursive::Cursive;
use cursive::view::TextView;

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(TextView::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean semper risus turpis, a blandit magna scelerisque vitae. Etiam urna dui, ullamcorper ornare libero eu, suscipit lacinia justo. Maecenas lobortis elit vel nibh elementum malesuada. Nam erat lorem, efficitur a tortor eget, mattis tempor nunc. Donec vel fringilla ipsum. Nullam consequat lacus nec feugiat consectetur. Etiam a ante aliquam, aliquet metus id, maximus enim. Maecenas lorem diam, gravida vitae varius suscipit, posuere nec nisl. Nunc non justo est. Ut sagittis est efficitur sapien mollis varius. Sed blandit magna in nibh pellentesque tincidunt. In arcu sem, egestas at odio vel, condimentum placerat lacus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nulla venenatis leo eu tellus consectetur tristique. Vestibulum pretium rutrum purus vitae rhoncus. Fusce eros libero, dignissim tristique lobortis et, tincidunt eu ex.

Quisque in bibendum nisi. Donec venenatis ligula dolor, sed malesuada ex tincidunt a. Vestibulum diam nulla, placerat a nibh non, dignissim mollis tortor. Donec lacinia tellus vel nulla mollis accumsan. Suspendisse vel massa vestibulum, aliquet tellus in, dignissim massa. Praesent feugiat viverra cursus. Maecenas eget urna leo. Nullam dui eros, egestas nec lacus vitae, ultrices vestibulum augue. Morbi vel massa eget lorem elementum dignissim eget sed risus. Curabitur ultricies nisl in erat pulvinar maximus. Donec at enim augue. Curabitur lacinia vestibulum risus, eget vulputate ante rhoncus quis.

Praesent iaculis enim id eros consectetur venenatis. Nunc malesuada faucibus luctus. Proin vel malesuada lacus, sed tempus risus. Quisque sagittis arcu dignissim ex pulvinar euismod. Vivamus elementum quis nibh sed rhoncus. Curabitur tristique elit id dolor rhoncus, et facilisis eros dictum. In a vulputate sapien, vitae tincidunt leo. Integer ornare enim elit, a pellentesque ex venenatis at. Pellentesque purus libero, posuere vel magna nec, imperdiet aliquam mi.

Sed consectetur imperdiet augue et consectetur. Aliquam rhoncus nisl et mi lobortis ultricies. Fusce pretium mi nibh, in maximus nunc porta et. Morbi maximus elit vel egestas sagittis. Pellentesque egestas vehicula augue. Quisque ornare placerat ante, non vulputate nisl placerat commodo. Nam sit amet eros placerat, varius diam a, bibendum sem. Mauris orci leo, dapibus nec sapien eu, finibus hendrerit erat. Duis et neque diam. Vivamus nec ultrices ipsum. In hac habitasse platea dictumst. Fusce vel eros sit amet velit sollicitudin eleifend vitae et nunc.

Donec elementum pharetra elit, quis finibus ante dictum et. Vivamus tristique dignissim nulla. Ut dignissim tincidunt elementum. Nullam pellentesque turpis sapien, iaculis sagittis arcu mollis nec. Pellentesque quis vehicula urna. Aenean pretium eget ex vitae interdum. Phasellus pharetra magna id vehicula cursus. Vivamus egestas id nulla et lacinia. Donec congue at eros volutpat tristique. Donec mi tortor, commodo a nisl sed, vulputate malesuada velit. Morbi sed sagittis odio. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Aenean lacinia auctor dapibus. Quisque sagittis odio vel urna laoreet ultrices."));

    // We can quit by pressing q
    siv.add_global_callback('q' as i32, |s| s.quit());

    siv.run();
}
