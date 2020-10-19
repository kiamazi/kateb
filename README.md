<div dir="rtl">

# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.


## نصب

### اوبونتو / دبیان و سایر نسخه‌های برپایه دبیان


- ‫در صورتی‌که از نسخه‌های قبل از اوبونتو ۲۰.۰۴ و یا سایر نسخه‌های بر پایه دبیان استفاده می‌کنید:

</div>

``` bash
$ sudo apt install make libssl-dev
$ sudo cpan -T kateb
```

<div dir="rtl">

- اگر از ابونتو 20.04 استفاده میکنید:
</div>

``` bash
$ sudo apt install make
$ sudo cpan -T kateb
```

<div dir="rtl">

### آرچ و نسخه‌های بر پایه‌ آن / مانجارو
</div>

``` bash
$ sudo pacman -S make
$ sudo cpan -T kateb
```

<div dir="rtl">

### فدورا
</div>

``` bash
$ sudo dnf install cpan make
$ sudo cpan -T kateb
```

<div dir="rtl">

### نصب از منبع کد
</div>

	$ git clone git@github.com:kiamazi/kateb.git
	$ cd kateb
	$ perl ./Makefile.PL
	$ make test                        #optional
	$ make install

<div dir="rtl">

# استفاده
</div>

	$ kateb <command> [option]

### fonts

<div dir="rtl">

نمایش فونت‌های پشتیبانی شده توسط کاتب
</div>

	$ kateb fonts


### install

<div dir="rtl">

نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده
</div>

	$ kateb install [font name(s) or all]

	$ kateb install vazir
	#or
	$ kateb install vazir sahel estedad
	#or
	$ kateb install all


### update

<div dir="rtl">

به روز رسانی فونت نصب شده
</div>

	$ kateb update [font name(s) or all]

	$ kateb update shabnam
	#or
	$ kateb update shabnam mikhad lalezar sahel
	#or
	$ kateb update all


### reinstall

<div dir="rtl">

دوباره نصب کردن فونت نصب شده
</div>

	$ kateb reinstall [font name(s) or all]

	$ kateb reinstall behdad
	#or
	$ kateb reinstall behdad ganjnameh parastoo
	#or
	$ kateb reinstall all


### list

<div dir="rtl">

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب
</div>

	$ kateb list


### info

<div dir="rtl">

خلاصه اطلاعات در مورد منتشر کننده فونت
</div>

	$ kateb info Mikhak
	#or
	$ kateb info all

<div dir="rtl">

## مسیر نصب

در صورتیکه فونت‌های پشتیبانی شده را قبلا در آدرس
</div>

	~/.local/share/fonts/

<div dir="rtl">

‫یا سایر مسیرهایی که به عنوان شاخه‌های فونت توسط سیستم‌عامل شما در دایرکتوری خانه پشتیبانی می‌شوند، کپی کرده‌اید ابتدا فونت‌های کپی شده را پاک کنید.

‫**۱-** نصب در دایرکتوری خانه کاربر:

‫پس از نصب در ترمینال تنها کافی است تایپ کنید:
</div>

	$ kateb <command>

<div dir="rtl">

‫مسیر نصب فونت‌ها توسط کاتب در پوشه‌ی خانه کاربر:
</div>

	~/.local/share/fonts/farsifreefont

<div dir="rtl">

‫**۲-** نصب سیستمی:
</div>

	$ sudo kateb <command>

<div dir="rtl">

‫مسیر نصب سیستمی فونت:
</div>

	/usr/share/fonts/truetype/farsifreefont

<div dir="rtl">

## فونت‌های پشتیبانی شده

لیست فونت‌هایی که در حال حاضر پشتیبانی می‌شوند
</div>

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
