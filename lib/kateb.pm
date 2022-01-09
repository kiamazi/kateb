package kateb;

use strict;
use warnings;
use 5.012;
our $VERSION = '01.00.28';

use kateb::Install;
use kateb::LocalData;

1;
__END__

=pod

=encoding utf8

=head1 NAME

kateb - tools for install and update farsi free fonts

=head1 VERSION

This document describes L<kateb> version B<01.00.28>.


=head1 kateb

kateb comes with a command-line application which you can use to install and update your fonts.

 $ cpanm kateb # Install

 $ kateb install [font name |or| all]      # install fonts in ~/.local/share/fonts/farsifreefonts

 $ kateb reinstall [font name |or| all]

 $ kateb update [font name |or| all]

 $ kateb list                                        # list of installed fonts

 $ kateb fonts                                    # list of available fonts

 $ sudo kateb install/reinstall/update [font name |or| all] # install fonts in /usr/share/fonts/farsifreefonts


=head1 Author

kateb was written by Kiavash Mazi
LL<mailto:kiavash@cpan.org>.


=head1 License and Copyright

kateb is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see LL<http://www.gnu.org/licenses/>.

=head1 فارسی

کاتب

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.

=head2 نصب

=head3 اوبونتو / دبیان و سایر نسخه‌های برپایه دبیان


- ‫در صورتی‌که از نسخه‌های قبل از اوبونتو ۲۰.۰۴ و یا سایر نسخه‌های بر پایه دبیان استفاده می‌کنید:

	sudo apt install make libssl-dev
	sudo cpan -T kateb


- اگر از ابونتو 20.04 استفاده میکنید:

	sudo apt install make
	sudo cpan -T kateb

=head3 آرچ و نسخه‌های بر پایه‌ آن / مانجارو

	sudo pacman -S make
	sudo cpan -T kateb

=head3 فدورا

	sudo dnf install cpan make perl-LWP-Protocol-https
	sudo cpan -T kateb

=head2 استفاده

kateb <command> [option]

=head3 fonts

نمایش فونت‌های پشتیبانی شده توسط کاتب

	kateb fonts

=head3 install

نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده

	kateb install [font name]

	kateb install vazir
	#or
	kateb install all

=head3 update

به روز رسانی فونت نصب شده

	kateb update shabnam
	#or
	kateb update all

=head3 reinstall

دوباره نصب کردن فونت نصب شده

	kateb reinstall behdad
	#or
	kateb reinstall all

=head3 list

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

	kateb list

=head3 info

خلاصه اطلاعات در مورد منتشر کننده فونت

	kateb info Mikhak
	#or
	kateb info all

=head3 self-upgrade

‫در صورت وجود داشتن نسخه جدیدتر کاتب، به کاربر اطلاع می‌دهد

=head2 مسیر نصب

‫**۱-** نصب در دایرکتوری خانه کاربر:

‫پس از نصب در ترمینال تنها کافی است تایپ کنید:

	kateb

در صورتیکه فونت‌های پشتیبانی شده را قبلا در آدرس

	~/.local/share/fonts/

‫یا سایر مسیرهایی که به عنوان شاخه‌های فونت توسط سیستم‌عامل شما در دایرکتوری خانه پشتیبانی می‌شوند، کپی کرده‌اید ابتدا فونت‌های کپی شده را پاک کنید.

‫مسیر نصب فونت‌ها توسط کاتب در پوشه‌ی خانه کاربر:

	~/.local/share/fonts/farsifreefont

‫**۲-** نصب سیستمی:

	sudo kateb

‫مسیر نصب سیستمی فونت:

	/usr/share/fonts/truetype/farsifreefont

=head2 فونت‌های پشتیبانی شده

لیست فونت‌هایی که در حال حاضر پشتیبانی می‌شوند

	behdad
	estedad
	farbod
	gandom
	ganjnameh
	lalezar
	mikhak
	nahid
	nika
	noon
	parastoo
	pfont
	sahel
	samim
	shabnam
	shahab
	tanha
	vazir
	vazircode
	ziracode
