macro_rules! draw_line {
    ( $viewport:expr, $camera:expr, $gl:expr,
      $r:expr, $g:expr, $b:expr, $a:expr,
      $x1:expr, $y1:expr, $x2:expr, $y2:expr ) => {{

        let line_drawer = Line {
            color: [$r,$g,$b,$a],
            radius: 1.,
            shape: line::Shape::Round,
        };

        let line = [$x1,$y1,$x2,$y2];

        $gl.draw(*$viewport, |context, gl| {
            let transform = $camera.trans(context.transform);

            line_drawer.draw(line, default_draw_state(), transform, gl);
        });
    }}
}
