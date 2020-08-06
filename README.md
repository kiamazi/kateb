# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.

## نصب

### اوبونتو / دبیان و سایر نسخه‌های برپایه دبیان


- ‫در صورتی‌که از نسخه‌های قبل از اوبونتو ۲۰.۰۴ و یا سایر نسخه‌های بر پایه دبیان استفاده می‌کنید:

``` bash
sudo apt install make libssl-dev
sudo cpan -T kateb
```

- اگر از ابونتو 20.04 استفاده میکنید:

``` bash
sudo apt install make
sudo cpan -T kateb
```

### آرچ و نسخه‌های بر پایه‌ آن / مانجارو

``` bash
sudo pacman -S make
sudo cpan -T kateb
```

### فدورا

``` bash
sudo dnf install cpan make
sudo cpan -T kateb
```

### نصب از منبع کد

	git clone git@github.com:kiamazi/kateb.git
	cd kateb
	perl ./Makefile.PL
	make test                        #optional
	make install

# استفاده

	kateb <command> [option]

### fonts

نمایش فونت‌های پشتیبانی شده توسط کاتب

	kateb fonts


### install

نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده

	kateb install [font name(s) or all]

	kateb install vazir
	#or
	kateb install vazir sahel estedad
	#or
	kateb install all


### update

به روز رسانی فونت نصب شده

	kateb update [font name(s) or all]

	kateb update shabnam
	#or
	kateb update shabnam mikhad lalezar sahel
	#or
	kateb update all


### reinstall

دوباره نصب کردن فونت نصب شده

	kateb reinstall [font name(s) or all]

	kateb reinstall behdad
	#or
	kateb reinstall behdad ganjnameh parastoo
	#or
	kateb reinstall all


### list

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

	kateb list


### info

خلاصه اطلاعات در مورد منتشر کننده فونت

	kateb info Mikhak
	#or
	kateb info all


## مسیر نصب

در صورتیکه فونت‌های پشتیبانی شده را قبلا در آدرس

	~/.local/share/fonts/

‫یا سایر مسیرهایی که به عنوان شاخه‌های فونت توسط سیستم‌عامل شما در دایرکتوری خانه پشتیبانی می‌شوند، کپی کرده‌اید ابتدا فونت‌های کپی شده را پاک کنید.

‫**۱-** نصب در دایرکتوری خانه کاربر:

‫پس از نصب در ترمینال تنها کافی است تایپ کنید:

	kateb <command>

‫مسیر نصب فونت‌ها توسط کاتب در پوشه‌ی خانه کاربر:

	~/.local/share/fonts/farsifreefont

‫**۲-** نصب سیستمی:

	sudo kateb <command>

‫مسیر نصب سیستمی فونت:

	/usr/share/fonts/truetype/farsifreefont


## فونت‌های پشتیبانی شده

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
	zira
