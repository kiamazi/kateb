package kateb;

use strict;
use warnings;
use 5.012;
our $VERSION = '01.00.00';


1;
__END__

=pod

=encoding utf8

=head1 NAME

kateb - tools for install and update farsi free fonts

=head1 VERSION

This document describes L<kateb> version B<00.98.12>.


=head1 kateb

kateb comes with a command-line application which you can use to install and update your fonts.

 $ cpanm kateb # Install

 $ kateb       # install fonts in ~/.local/share/fonts/farsifreefonts

 $ sudo kateb  # install fonts in /usr/share/fonts/farsifreefonts


=head1 Author

kateb was written by Kiavash Mazi
LL<mailto:kiavash@cpan.org>.


=head1 License and Copyright

kateb is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see LL<http://www.gnu.org/licenses/>.

=cut
