<?php

function calculate(int $serial, int $x, int $y): int {
	$rack_id = $x + 10;
    $power = ($rack_id * $y + $serial) * $rack_id;
    $without_tail = ($power - ($power % 100)) / 100;

    return ($without_tail % 10) - 5;
}

/*
echo calculate(8, 3, 5) . PHP_EOL;
echo calculate(57, 122, 79) . PHP_EOL;
echo calculate(39, 217, 196) . PHP_EOL;
echo calculate(71, 101, 153) . PHP_EOL;
*/


$serial = 4172;
$size = 300;

$field = [];
for ($i = 0; $i < $size; $i++) {
	$field[$i] = [];
}

$now = microtime(true);

echo "Calculate the base grid … ";
for ($y = 0; $y < $size; $y++) {
	for ($x = 0; $x < $size; $x++) {
		$field[$y][$x] = calculate($serial, $x + 1, $y + 1);
	}
}

$diff = round(microtime(true) - $now, 3);
echo "finished in $diff seconds.\n";


echo "Creating sums … ";
$now = microtime(true);
for ($y = 0; $y < $size; $y++) {
	for ($x = 0; $x < $size; $x++) {
		$sum = $field[$y][$x];
		if ($x > 0) {
			$sum = $sum + $field[$y][$x - 1];
		}
		if ($y > 0) {
			$sum = $sum + $field[$y - 1][$x];
		}
		if ($x > 0 && $y > 0) {
			$sum = $sum - $field[$y - 1][$x - 1];
		}
		$field[$y][$x] = $sum;
	}
}
$diff = round(microtime(true) - $now, 3);
echo "finished in $diff seconds.";


echo "Searching for maximum area … ";
$max_total = -INF;
$max_position_x = 0;
$max_position_y = 0;
$max_total_size = 0;
$now = microtime(true);

for ($square_size = 0; $square_size < $size; $square_size++) {
	echo "Checking " . ($square_size + 1) . " …";
	$max_size = $size + 1 - ($square_size + 1);
	for ($y = 0; $y < $max_size; $y++) {
		for ($x = 0; $x < $max_size; $x++) {
			$a = 0;
			if ($y > 0 && $x > 0) {
				$a = $field[$y - 1              ][$x - 1              ];
			}
			$b = 0;
			if ($y > 0) {
				$b = $field[$y - 1              ][$x     + $square_size];
			}
			$c = 0;
			if ($x > 0) {
				$c = $field[$y     + $square_size][$x - 1              ];
			}
			$d = $field[$y     + $square_size][$x     + $square_size];

			$total = $a - $b - $c + $d;

			if ($square_size == 1) {
				$total = $field[$y][$x];
			}
			/* end of new way */

			if ($total > $max_total) {
				$max_total = $total;
				$max_position_x = $x + 1;
				$max_position_y = $y + 1;
				$max_total_size = $square_size + 1;
			}
		}
	}
	$diff = round(microtime(true) - $now, 3);
	echo "(local) Position: $max_position_x $max_position_y, Total: $max_total, Size: $max_total_size, Time: $diff seconds\n";
}

$diff = round(microtime(true) - $now, 3);
echo "Position: $max_position_x $max_position_y, Total: $max_total, Size: $max_total_size, Time: $diff seconds\n";

