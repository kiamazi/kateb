# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.

## نصب

	sudo cpan kateb

### پیش‌نیازها
- ‫در صورتی‌که از اوبونتو و یا سایر نسخه‌های بر پایه دبیان استفاده می‌کنید و openssl و یا کتابخانه‌های آن نصب نشده‌اند:

``` bash
sudo apt install openssl libssl-dev
```

- آرچ و نسخه‌های بر پایه‌ی آن مانند مانجارو

> نیازی به نصب پیش نیاز ندارید

## استفاده

### fonts
نمایش فونت‌های پشتیبانی شده توسط کاتب

	kateb fonts

### install
نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده

	kateb install [font name]

	kateb install vazir
	#or
	kateb install all

### update
به روز رسانی فونت نصب شده

	kateb update shabnam
	#or
	kateb update all

### reinstall
دوباره نصب کردن فونت نصب شده

	kateb reinstall behdad
	#or
	kateb reinstall all

### list
لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

	kateb list

## مسیر نصب
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

## فونت‌های پشتیبانی شده
لیست فونت‌هایی که در حال حاضر پشتیبانی می‌شوند

	behdad
	farbod
	gandom
	ganjnameh
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
