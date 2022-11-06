exports.keys = 'SJBWYLBSLOVEPEACELONGLIVECHINESE';

exports.view = {
    defaultViewEngine: 'nunjucks',
    mapping: {
        '.tpl': 'nunjucks'
    }
};

// config/config.default.js
// 添加 news 的配置项
exports.news = {
    pageSize: 5,
    serverUrl: 'https://hacker-news.firebaseio.com/v0',
};

// config/config.default.js
// add middleware robot
exports.middleware = ['robot'];
// robot's configurations
exports.robot = {
    ua: [/Baiduspider/i],
};