# School Clock

> The clock always ran slowest just before the bell rang. Minutes would pass between each tick of the second hand, while the teacher droned interminably in the background. It was only natural that it became an object of fascination: how could it vary its rate so dramatically, while agreeing with our parents' watches at the beginning and end of each day?

This project contains a school clock: one which runs slightly fast for the first portion of a school period, and then compensates by slowing down during the last bit. It is, of course, completely configurable.

## Usage

The clock is configured by four variables: `beat`, `inflection`, `fade`, and `initialize`.

- `beat` is the period over which the clock repeats its cycle.
- `inflection` is the point in real time at which the clock stops running fast and begins running slow. This must be smaller than `beat`.
- `fade` is the reported time at the `inflection` point. It must be between `inflection` and `beat`.
- `initialize` is the point in real time at which the clock starts.

## Example

Your school begins first period at 0830 daily. Class periods are one hour long. You therefore set `initialize` to 0830 and `beat` to one hour. You desire to have a fairly dramatic slowdown, so you set `inflection` to 54 minutes and `fade` to 59 minutes. Your clocks will then read as follows:

| Real Time | Reported Time |
| :----: | :----: |
| 08:30:00 | 08:30:00 |
| 08:57:00 | 08:59:30 |
| 09:24:00 | 09:29:00 |
| 09:27:00 | 09:29:30 |
| 09:30:00 | 09:30:00 |

This cycle of speeded and slowed time will then repeat indefinitely. At all times not recorded in the table, the reported time is linearly interpolated appropriately.

## Attribution

<blockquote class="twitter-tweet" data-lang="en"><p lang="en" dir="ltr">You know what would be fun to install in schools? An analog clock that runs slightly faster for most of the hour, and then slows down by a factor of 3 when the minute hand gets to X:58, so it takes 6 minutes to get to :00. (Ideally adjustable, in case the class ends at e.g. :35.)</p>&mdash; Eliezer Yudkowsky (@ESYudkowsky) <a href="https://twitter.com/ESYudkowsky/status/1107812511087095808?ref_src=twsrc%5Etfw">March 19, 2019</a></blockquote>
<script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
