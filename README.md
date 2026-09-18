# Persian fonts installer

a script to download, install and update Persian free(libre) fonts on all GNU/Linux distros

# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.

## نصب

```
cargo install --git https://github.com/kiamazi/kateb 
```

یا نسخه باینری را مستقیما از (صفحه انتشار)[https://github.com/kiamazi/kateb/releases] دانلود کنید

## نصب از منبع کد

```
git clone git@github.com:kiamazi/kateb.git
cd kateb
cargo build --release
```

> [!NOTE]
> ‫اگر از یکی از نسخه‌های قدیمی‌تر از نسخه۲(نسخه‌های perl) استفاده می‌کنید یا می‌کردید، لطفا
> ابتدا دایرکتوری قدیمی فونت‌ها را از حذف کنید. در یکی از این ۴مسیر(بسته به سیستم عامل)
> دایرکتوری `farsifreefont` را پیدا کنید و آن را حذف کنید 
```
# mac:
/Library/Fonts/farsifreefont
~/Library/Fonts/farsifreefont

# GNU/linux
/usr/share/fonts/truetype/farsifreefont
~/.local//share/fonts/farsifreefont
```

# استفاده

    kateb <command> [option]

### list

نمایش فونت‌های پشتیبانی شده توسط کاتب

    kateb list

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

### uninstall

حذف کردن فونت نصب شده

    kateb uninstall [font name(s) or all]

    kateb uninstall ario
    #or
    kateb uninstall nika arad ziracode
    #or
    kateb uninstall all


### fonts

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

    kateb fonts

### info

خلاصه اطلاعات در مورد منتشر کننده فونت

    kateb info Mikhak
    #or
    kateb info all

### help

```
~$ kateb help

Usage: kateb <COMMAND>

Commands:
  install    Install a new font
  update     Update an installed font
  reinstall  Reinstall an already-installed font
  uninstall  Uninstall an already-installed font
  list       List all supported Farsi fonts
  fonts      Show the fonts that are currently installed
  info       Display brief information about a font's publisher
  version    Display the kateb version [alias: -v]
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## مسیر نصب

در صورتیکه فونت‌های پشتیبانی شده را قبلا در آدرس

    ~/.local/share/fonts/
    ~/.local/share/fonts/farsifreefont

‫یا سایر مسیرهایی که به عنوان شاخه‌های فونت توسط سیستم‌عامل شما در دایرکتوری خانه پشتیبانی می‌شوند، کپی یا با کمک نسخه‌های قبل از۲ کاتب نصب کرده‌اید، ابتدا فونت‌های کپی شده را پاک کنید.

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

    arad
    ario
    behdad
    estedad
    farbod
    gandom
    ganjnameh
    lalezar
    mikhak
    nahid
    nastaliq
    nika
    noon
    parastoo
    pfont
    rooyin
    sahel
    samim
    shabnam
    shahab
    tanha
    vazir
    vazircode
    ziracode
