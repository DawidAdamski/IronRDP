//! Conversion of mouse input PDUs into [`MouseEvent`].

use ironrdp_pdu::input::MousePdu;
use ironrdp_pdu::input::mouse::PointerFlags;
use ironrdp_server::MouseEvent;

fn mouse(flags: PointerFlags, wheel: i16) -> MouseEvent {
    MousePdu {
        flags,
        number_of_wheel_rotation_units: wheel,
        x_position: 100,
        y_position: 200,
    }
    .into()
}

/// PTRFLAGS_BUTTON3 with PTRFLAGS_DOWN is a middle button press, not a move.
#[test]
fn middle_button_down_is_a_middle_press() {
    let event = mouse(PointerFlags::MIDDLE_BUTTON_OR_WHEEL | PointerFlags::DOWN, 0);
    assert!(
        matches!(event, MouseEvent::MiddlePressed),
        "expected MiddlePressed, got {event:?}"
    );
}

/// The same flag without PTRFLAGS_DOWN is the release.
#[test]
fn middle_button_up_is_a_middle_release() {
    let event = mouse(PointerFlags::MIDDLE_BUTTON_OR_WHEEL, 0);
    assert!(
        matches!(event, MouseEvent::MiddleReleased),
        "expected MiddleReleased, got {event:?}"
    );
}

/// A wheel event stays a wheel event even when the client also sets
/// PTRFLAGS_BUTTON3, whose name ("middle button or wheel") invites that.
#[test]
fn a_wheel_rotation_is_not_a_middle_click() {
    let event = mouse(PointerFlags::VERTICAL_WHEEL | PointerFlags::MIDDLE_BUTTON_OR_WHEEL, -3);
    assert!(
        matches!(event, MouseEvent::VerticalScroll { value: -3 }),
        "expected VerticalScroll, got {event:?}"
    );
}

/// The left and right buttons, and a plain move, are unaffected.
#[test]
fn the_other_buttons_and_moves_are_unchanged() {
    assert!(matches!(
        mouse(PointerFlags::LEFT_BUTTON | PointerFlags::DOWN, 0),
        MouseEvent::LeftPressed
    ));
    assert!(matches!(mouse(PointerFlags::LEFT_BUTTON, 0), MouseEvent::LeftReleased));
    assert!(matches!(
        mouse(PointerFlags::RIGHT_BUTTON | PointerFlags::DOWN, 0),
        MouseEvent::RightPressed
    ));
    assert!(matches!(
        mouse(PointerFlags::RIGHT_BUTTON, 0),
        MouseEvent::RightReleased
    ));
    assert!(matches!(
        mouse(PointerFlags::MOVE, 0),
        MouseEvent::Move { x: 100, y: 200 }
    ));
}
