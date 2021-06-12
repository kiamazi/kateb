<div dir="rtl">

# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.

## نصب

### اوبونتو / دبیان و سایر نسخه‌های برپایه دبیان


- ‫در صورتی‌که از نسخه‌های قبل از اوبونتو ۲۰.۰۴ و یا سایر نسخه‌های بر پایه دبیان استفاده می‌کنید:

<div dir="ltr">

``` bash
sudo apt install make libssl-dev
sudo cpan -T kateb
```

</div>

- اگر از ابونتو 20.04 استفاده میکنید:

<div dir="ltr">

``` bash
sudo apt install make
sudo cpan -T kateb
```

</div>

### آرچ و نسخه‌های بر پایه‌ آن / مانجارو

<div dir="ltr">

``` bash
sudo pacman -S make
sudo cpan -T kateb
```

</div>

### فدورا

<div dir="ltr">

``` bash
sudo dnf install cpan make
sudo cpan -T kateb
```

</div>

### نصب از منبع کد

<div dir="ltr">

``` bash
git clone git@github.com:kiamazi/kateb.git
cd kateb
perl ./Makefile.PL
make test                        #optional
make install
```

</div>

# استفاده

<div dir="ltr">

``` bash
kateb <command> [option]
```

</div>

### fonts

نمایش فونت‌های پشتیبانی شده توسط کاتب

<div dir="ltr">

``` bash
kateb fonts
```

</div>


### install

نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده

<div dir="ltr">

``` bash
kateb install [font name(s) or all]

kateb install vazir
#or
kateb install vazir sahel estedad
#or
kateb install all
```

</div>


### update

به روز رسانی فونت نصب شده

<div dir="ltr">

``` bash
kateb update [font name(s) or all]

kateb update shabnam
#or
kateb update shabnam mikhad lalezar sahel
#or
kateb update all
```

</div>


### reinstall

دوباره نصب کردن فونت نصب شده

<div dir="ltr">

``` bash
kateb reinstall [font name(s) or all]

kateb reinstall behdad
#or
kateb reinstall behdad ganjnameh parastoo
#or
kateb reinstall all
```

</div>


### list

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

<div dir="ltr">

``` bash
kateb list
```

</div>


### info

خلاصه اطلاعات در مورد منتشر کننده فونت

<div dir="ltr">

``` bash
kateb info Mikhak
#or
kateb info all
```

</div>

### self-upgrade

‫در صورت وجود داشتن نسخه جدیدتر کاتب، به کاربر اطلاع می‌دهد

<div dir="ltr">

``` bash
kateb self-upgrade
```

</div>

## مسیر نصب

در صورتیکه فونت‌های پشتیبانی شده را قبلا در آدرس

<div dir="ltr">

``` bash
~/.local/share/fonts/
```

</div>

‫یا سایر مسیرهایی که به عنوان شاخه‌های فونت توسط سیستم‌عامل شما در دایرکتوری خانه پشتیبانی می‌شوند، کپی کرده‌اید ابتدا فونت‌های کپی شده را پاک کنید.

‫**۱-** نصب در دایرکتوری خانه کاربر:

‫پس از نصب در ترمینال تنها کافی است تایپ کنید:

<div dir="ltr">

``` bash
kateb <command>

```

</div>

‫مسیر نصب فونت‌ها توسط کاتب در پوشه‌ی خانه کاربر:

<div dir="ltr">

``` bash
~/.local/share/fonts/farsifreefont
```

</div>

‫**۲-** نصب سیستمی:

<div dir="ltr">

``` bash
sudo kateb <command>
```

</div>

‫مسیر نصب سیستمی فونت:

<div dir="ltr">

``` bash
/usr/share/fonts/truetype/farsifreefont
```

</div>


## فونت‌های پشتیبانی شده

لیست فونت‌هایی که در حال حاضر پشتیبانی می‌شوند

<div dir="ltr">

``` bash
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
```

</div>

</div>
