use scraping_sample::wixoss::{Spell, WixossCard, Card, CardType};

fn main() {
    let source: String = r#"

    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-061</p>
                                <p class="cardName">TEMPO　UP<br class="sp"><span>＜テンポアップ＞</span></p>
                                <div class="cardRarity">C</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-061.jpg">
                                                                <p>Illust <span>茶ちえ</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>スペル</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>青</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《青》×１</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>ガード</dt>
                                    <dd>-</dd>

                                    <dt>フォーマット</dt>
                                    <dd><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_key.png" height="23" alt="《キーアイコン》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_diva.png" height="23" alt="《ディーヴァアイコン》" /></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        あなたの青のシグニ１体を対象とし、ターン終了時まで、それは「<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" />：このシグニがアタックしたとき、対戦相手のセンタールリグのレベル以下の数字１つを宣言する。対戦相手の手札を見て、<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_guard_mini.png" height="23" alt="《ガードアイコン》" />を持たず宣言した数字と同じレベルを持つすべてのシグニを捨てさせる。」を得る。それが《コードハート　ピルルク//フェゾーネ》の場合、それは覚醒する。（シグニは覚醒すると場にあるかぎり覚醒状態になる）                                    </div>
                                                                                                    <div class="cardSkill">
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_burst.png" width="26" height="24" alt="ライフバースト" />：対戦相手のシグニを２体まで対象とし、それらをダウンする。                                    </div>

                                                                    <div class="cardText mb20">
                                        「アゲアゲ…?」～ピルルク～                                    </div>

                                                                                            </div>
                        </div>
                    </div>
                </section>

        </main><!-- .site-main -->
    </div><!-- .content-area -->

    <script>
        $(function() {
            // //サブメニューナビゲーション
            // $('.accordionTrg').click(function () {
            //     $('.accordion').slideToggle();
            //     console.log('detail.php');
            //     $(this).toggleClass('opn');
            // });
            // //チェックすべて外す
            // $('#noncheck').click(function () {
            //     $('.cardform input[type="checkbox"]').prop('checked', false);
            // });
            /*
            $('.cboxElement').click(function () {
              $('.mordal').css('display', 'block');
              $('body,html').css('overflow', 'hidden');
            });*/
            $('.mordal .close').click(function () {
                /*$('.mordal').css('display', 'none');
                $('body,html').css('overflow', 'auto');*/
                parent.$.fn.colorbox.close(); return false;
                //console.log("ここ");
            });
        });
    </script>

    <!-- /新デザイン -->
    </body>
    </html>

"#.into();

    let spell = Spell::from_source(source);
    println!("{}", &spell);
    let card: Card = spell.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::Spell);
}
