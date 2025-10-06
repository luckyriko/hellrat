import { createI18n } from 'vue-i18n'
import enUS from './locales/en-US.json'
import zhCN from './locales/zh-CN.json'

const messages = {
    'en-US': enUS,
    'zh-CN': zhCN
}

// 获取浏览器语言设置
const navigatorLang = navigator.language || 'en-US';
const defaultLang = Object.keys(messages).includes(navigatorLang)
    ? navigatorLang
    : 'en-US';

// 创建 i18n 实例
const i18n = createI18n({
    legacy: false,
    locale: localStorage.getItem('Language') || defaultLang,
    fallbackLocale: 'en-US',
    messages
})

export default i18n
