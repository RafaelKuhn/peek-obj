
pub const HELP_SCR: &[u8] = br#"


W / S: move camera forwards / backwards
A / D: move camera left / right
E / Q: move camera up / down
R: reset camera position and orientation

V: toggle verbose mode, hides the UI text

M: toggle camera movement mode from
		orbital to free camera

arrow keys: in free camera mode, changes the
		direction the camera is looking

T: take screenshot, saves a .txt dump of the screen
		in the following path: "screenshot.txt"

C: change culling mode, can cull balls, triangles
		or none
Z: change Z-sorting mode, can render all triangles
		after all of the spheres and vice-versa
L: change spheres lighting mode, can be by index,
		by camera distance or by height

SHIFT + C / L / Z: the same but in reverse order

P: pauses / unpauses the engine, useful to copy
		parts of the screen in some terminals

G: toggles rendering of the XYZ world axis
		(renders after everything else)

H: enters / quits help screen
"#;